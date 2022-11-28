use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

use const_str::concat;
use serde::Serialize;

use crate::client::DOCKER_ENGINE_VERSION;
use crate::errors::DecCreateError;
use crate::imp::url::UrlBuilder;
use crate::imp::url_parser::is_http;
use crate::requests::{BuildImageRequest, CreateImageRequest, ListContainersRequest, LogsArgs, RemoveContainerArgs, WaitCondition};

pub(crate) const DOCKER_ENGINE_VERSION_PATH: &str = concat!("/", DOCKER_ENGINE_VERSION);

/// Docker Engine connection reference:
///
/// https://docs.docker.com/desktop/faqs/general/
#[derive(Clone, Debug)]
pub(crate) enum DockerEngineApiBase {
    /// Explicit HTTP or HTTPS
    Http(String),

    /// Implicit HTTP. This is common on Windows, such as tcp://localhost:2375
    Tcp {
        display_url: String,
        implied_url: String
    },

    /// A UNIX socket path
    #[cfg(unix)]
    Unix(String)
}

impl DockerEngineApiBase {
    pub fn display_url(&self) -> &String {
        match self {
            DockerEngineApiBase::Http(url) => url,
            DockerEngineApiBase::Tcp { display_url, implied_url: _ } => display_url,

            #[cfg(unix)]
            DockerEngineApiBase::Unix(socket) => socket
        }
    }

    pub fn implied_url(&self) -> SchemedUrl {
        match self {
            DockerEngineApiBase::Http(url) =>
                if url.starts_with("https:") {
                    SchemedUrl::Https(url.clone())
                }
                else {
                    SchemedUrl::Http(url.clone())
                },

            DockerEngineApiBase::Tcp { display_url: _display_url, implied_url } =>
                SchemedUrl::Http(implied_url.clone()),

            #[cfg(unix)]
            DockerEngineApiBase::Unix(url) =>
                SchemedUrl::Unix(url.clone())
        }
    }
}

impl Display for DockerEngineApiBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_url())
    }
}

#[derive(Debug)]
pub(crate) enum DockerEngineApiBuilderError {
    Json(serde_json::Error),
    Url(url::ParseError)
}

impl From<serde_json::Error> for DockerEngineApiBuilderError {
    fn from(other: serde_json::Error) -> Self {
        DockerEngineApiBuilderError::Json(other)
    }
}

impl From<url::ParseError> for DockerEngineApiBuilderError {
    fn from(other: url::ParseError) -> Self {
        DockerEngineApiBuilderError::Url(other)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct DockerEngineServer {
    pub base: DockerEngineApiBase
}

impl DockerEngineServer {

    fn strip_unix(uri: &str) -> Option<String> {
        if uri.starts_with('/') {
            Some(uri.to_string())
        }
        else {
            uri
                .strip_prefix("unix://")
                .map(|v| v.to_string())
        }
    }

    pub fn new<S: Into<String>>(uri_or_unix_socket: S) -> Result<Self, DecCreateError> {
        Ok(Self {
            base: Self::select_base(uri_or_unix_socket)?
        })
    }

    fn select_base<S: Into<String>>(uri_or_unix_socket: S) -> Result<DockerEngineApiBase, DecCreateError> {
        let server = uri_or_unix_socket.into();

        match Self::strip_unix(&server) {
            Some(path) => {
                #[cfg(unix)]
                {
                    Ok(DockerEngineApiBase::Unix(path))
                }
                #[cfg(not(unix))]
                Err(DecCreateError::NixPlatformFeatureDisabled(path))
            }
            None => {
                match server.strip_prefix("tcp://") {
                    Some(stripped) => {
                        Ok(DockerEngineApiBase::Tcp {
                            display_url: server.clone(),
                            implied_url: format!("http://{}", stripped)
                        })
                    }
                    None => {
                        if is_http(&server) {
                            Ok(DockerEngineApiBase::Http(server))
                        }
                        else {
                            Err(DecCreateError::UnsupportedUrlScheme)
                        }
                    }
                }
            }
        }
    }

    fn as_string(&self) -> String {
        match &self.base {
            DockerEngineApiBase::Http(url) => {
                url.to_string()
            },
            DockerEngineApiBase::Tcp { implied_url, ..} => {
                implied_url.to_string()
            },
            #[cfg(unix)]
            DockerEngineApiBase::Unix(path) => {
                let uri: hyper::Uri = hyperlocal::Uri::new(path, "").into();
                uri
                    .to_string()
                    .trim_end_matches('/')
                    .to_string()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct DockerEngineApi {
    // Validated and cleaned URL without trailing forward slash
    base: String,

    // Validated but original URL provided by application.
    //
    // Windows clients may provide a tcp:// url that connects to a http server,
    // and Unix clients may provide a user-friendly path that is resolved to a file descriptor URL.
    display_url: String
}

impl Display for DockerEngineApi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Docker engine at {}", self.display_url)
    }
}

impl DockerEngineApi {
    pub(crate) fn new(server: DockerEngineServer) -> Self {
        Self {
            base: format!("{}{}", server.as_string(), DOCKER_ENGINE_VERSION_PATH),
            display_url: server.base.display_url().clone()
        }
    }

    /// Only for testing url builders
    #[allow(dead_code)]  // Dead code detector doesn't realize its used by a test
    fn without_version(mut self) -> Self {
        self.base = self.base.strip_suffix(DOCKER_ENGINE_VERSION_PATH)
            .unwrap()
            .to_string();

        self
    }

    pub(crate) fn with_server(server: String) -> Result<Self, DecCreateError> {
        Ok(Self::new(DockerEngineServer::new(server)?))
    }

    // For testing only, to make path expectations shorter
    #[allow(dead_code)]  // Dead code detector doesn't realize its used by a test
    fn without_server() -> Self {
        Self {
            base: "".into(),
            display_url: "".into()
        }
    }

    fn at(&self, path: String) -> String {
        format!("{}{}", self.base, path)
    }

    fn builder(&self) -> Result<UrlBuilder, url::ParseError> {
        UrlBuilder::from_str(&self.base)
    }

    pub fn containers(&self) -> DockerEngineApiPathContainers {
        DockerEngineApiPathContainers { base: self.clone() }
    }

    pub fn exec(&self) -> DockerEngineApiPathExec {
        DockerEngineApiPathExec { base: self.clone() }
    }

    pub fn images(&self) -> DockerEngineApiPathImages {
        DockerEngineApiPathImages { base: self.clone() }
    }

    pub fn networks(&self) -> DockerEngineApiPathNetworks {
        DockerEngineApiPathNetworks { base: self.clone() }
    }

    pub fn version(&self) -> String {
        self.at("/version".into())
    }

    pub fn volumes(&self) -> DockerEngineApiPathVolumes {
        DockerEngineApiPathVolumes { base: self.clone() }
    }
}

pub(crate) struct DockerEngineApiPathContainers {
    base: DockerEngineApi
}

impl DockerEngineApiPathContainers {
    pub fn create(&self, name: Option<&String>) -> String {
        self.base.at(
            format!(
                "/containers/create{}",
                name
                    .map(|n| format!("?name={}", n))
                    .unwrap_or_default()
            )
        )
    }

    pub fn create_exec<ID: Into<String>>(&self, name_or_id: ID) -> String {
        self.base.at(format!("/containers/{}/exec", name_or_id.into()))
    }

    pub fn files<ID: Into<String>>(&self, container_name_or_id: ID) -> DockerEngineApiPathContainerFiles {
        DockerEngineApiPathContainerFiles {
            base: self.base.clone(),
            container_name_or_id: container_name_or_id.into()
        }
    }

    pub fn inspect<ID: Into<String>>(&self, name_or_id: ID, size: bool) -> String {
        self.base.at(format!("/containers/{}/json?size={}", name_or_id.into(), size))
    }

    pub fn kill<ID: Into<String>>(&self, name_or_id: ID, signal: Option<String>) -> Result<String, url::ParseError> {
        Ok(self.base.builder()?
            .join("containers")?
            .join(&name_or_id.into())?
            .join("kill")?
            .query()
            .option("signal", signal)
            .to_string()
        )
    }

    pub fn list(&self, args: ListContainersRequest) -> Result<String, DockerEngineApiBuilderError> {
        let filters = if args.filters.is_empty() {
            None
        }
        else {
            Some(serde_json::to_string(&args.filters)?)
        };

        let builder = self.base.builder()?
            .join("containers")?
            .join("json")?
            .query()
            .option("all", args.all)
            .option("limit", args.limit)
            .option("size", args.size)
            .option("filters", filters);

        Ok(builder.to_string())
    }

    pub fn logs<ID: Into<String>>(&self, name_or_id: ID, args: LogsArgs) -> Result<String, url::ParseError> {
        Ok(self.base.builder()?
           .join("containers")?
           .join(&name_or_id.into())?
           .join("logs")?
           .query()
           .append("stdout", args.stdout)
           .append("stderr", args.stderr)
           .append("timestamps", args.timestamps)
           .to_string()
        )
    }

    #[cfg(not(windows))]  // Docker for Windows does not support pausing containers.
    pub fn pause<ID: Into<String>>(&self, name_or_id: ID) -> String {
        self.base.at(format!("/containers/{}/pause", name_or_id.into()))
    }

    pub fn rename<ID: Into<String>, NN: Into<String>>(&self, name_or_id: ID, new_name: NN) -> String {
        self.base.at(format!("/containers/{}/rename?name={}", name_or_id.into(), new_name.into()))
    }

    pub fn remove<ID: Into<String>>(&self, name_or_id: ID, args: RemoveContainerArgs) -> Result<String, url::ParseError> {
        Ok(self.base.builder()?
            .join("containers")?
            .join(&name_or_id.into())?
            .query()
            .option("force", args.force)
            .option("link", args.remove_link)
            .option("v", args.remove_volumes)
            .to_string()
        )
    }

    pub fn start<ID: Into<String>>(&self, name_or_id: ID) -> String {
        self.base.at(format!("/containers/{}/start", name_or_id.into()))
    }

    pub fn stop<ID: Into<String>>(&self, name_or_id: ID) -> String {
        self.base.at(format!("/containers/{}/stop", name_or_id.into()))
    }

    pub fn top<ID: Into<String>>(&self, name_or_id: ID, ps_args: Option<String>) -> Result<String, url::ParseError> {
        Ok(self.base.builder()?
            .join("containers")?
            .join(&name_or_id.into())?
            .join("top")?
            .query()
            .option("ps_args", ps_args)
            .to_string())
    }

    #[cfg(not(windows))]  // Docker for Windows does not support pausing containers.
    pub fn unpause<ID: Into<String>>(&self, name_or_id: ID) -> String {
        self.base.at(format!("/containers/{}/unpause", name_or_id.into()))
    }

    pub fn wait<ID: Into<String>>(&self, name_or_id: ID, condition: WaitCondition) -> String {
        let value = match condition {
            WaitCondition::NotRunning => "not-running",
            WaitCondition::NextExit => "next-exit",
            WaitCondition::Removed => "removed",
        };

        self.base.at(format!("/containers/{}/wait?condition={}", name_or_id.into(), value))
    }
}

pub(crate) struct DockerEngineApiPathExec {
    base: DockerEngineApi
}

impl DockerEngineApiPathExec {

    pub fn inspect<ID: Into<String>>(&self, id: ID) -> String {
        self.base.at(format!("/exec/{}/json", id.into()))
    }

    pub fn start<ID: Into<String>>(&self, id: ID) -> String {
        self.base.at(format!("/exec/{}/start", id.into()))
    }

}


pub(crate) struct DockerEngineApiPathContainerFiles {
    base: DockerEngineApi,
    container_name_or_id: String
}

impl DockerEngineApiPathContainerFiles {

    pub fn changes(&self) -> String {
        self.base.at(format!("/containers/{}/changes", self.container_name_or_id))
    }

    pub fn get<P: Into<String>>(&self, path: P) -> String {
        self.base.at(format!("/containers/{}/archive?path={}", self.container_name_or_id, path.into()))
    }

}

pub(crate) struct DockerEngineApiPathImages {
    base: DockerEngineApi
}

impl DockerEngineApiPathImages {

    fn sz_map<K, V>(value: &HashMap<K, V>) -> Result<Option<String>, serde_json::Error>
    where
        K: Serialize + Eq + Hash,
        V: Serialize
    {
        if value.is_empty() {
            Ok(None)
        }
        else {
            serde_json::to_string(value)
                .map(Some)
        }
    }

    fn sz_vec<A>(value: &Vec<A>) -> Result<Option<String>, serde_json::Error>
    where
        A: Serialize
    {
        if value.is_empty() {
            Ok(None)
        }
        else {
            serde_json::to_string(value)
                .map(Some)
        }
    }

    pub fn build(&self, request: BuildImageRequest) -> Result<String, DockerEngineApiBuilderError> {
        Ok(self.base.builder()?
            .join("build")?
            .query()
            .option("dockerfile", request.dockerfile)
            .append_all("t", request.tags)
            .option("extrahosts", request.extra_hosts)
            .option("remote", request.remote)
            .option("q", request.quiet)
            .option("nocache", request.no_cache)
            .option("cachefrom", Self::sz_vec(&request.cache_from)?)
            .option("pull", request.pull)
            .option("rm", request.remove_intermediates)
            .option("forcerm", request.force_remove_intermediates)
            .option("memory", request.memory_limit)
            .option("memswap", request.memory_and_swap)
            .option("cpushares", request.cpu_shares)
            .option("cpusetcpus", request.cpu_set_cpus)
            .option("cpuperiod", request.cpu_period)
            .option("cpuquota", request.cpu_quota)
            .option("buildargs", Self::sz_map(&request.build_args)?)
            .option("shmsize", request.shm_size_bytes)
            .option("squash", request.squash)
            .option("labels", Self::sz_map(&request.labels)?)
            .option("networkmode", request.network_mode)
            .option("platform", request.platform)
            .option("target", request.target)
            .option("outputs", request.outputs)
            .to_string()
        )
    }

    pub fn create(&self, request: CreateImageRequest) -> Result<String, url::ParseError> {
        Ok(self.base.builder()?
            .join("images/create")?
            .query()
            .option("fromImage", request.from_image)
            .option("fromSrc", request.from_src)
            .option("repo", request.repo)
            .option("tag", request.tag)
            .option("message", request.message)
            .option("platform", request.platform)
            .to_string()
        )
    }

    pub fn list(&self) -> String {
        self.base.at("/images/json".into())
    }

    pub fn push<R: Into<String>, T: Into<String>>(&self, repo: R, tag: T) -> String {
        self.base.at(format!("/images/{}/push?tag={}", repo.into(), tag.into()))
    }

    pub fn tag<ID: Into<String>, R: Into<String>, T: Into<String>>(&self, image_id_or_name_and_tag: ID, new_repo: R, new_tag: T) -> String {
        self.base.at(format!("/images/{}/tag?repo={}&tag={}", image_id_or_name_and_tag.into(), new_repo.into(), new_tag.into()))
    }

    pub fn untag<ID: Into<String>>(&self, image_id_or_name_and_tag: ID) -> String {
        self.base.at(format!("/images/{}", image_id_or_name_and_tag.into()))
    }
}

pub(crate) struct DockerEngineApiPathNetworks {
    base: DockerEngineApi
}

impl DockerEngineApiPathNetworks {

    pub fn create(&self) -> String {
        self.base.at("/networks/create".into())
    }

    pub fn inspect<ID: Into<String>, S: Into<String>>(&self, name_or_id: ID, scope: Option<S>, verbose: bool) -> String {
        self.base.at(
            format!(
                "/networks/{}?verbose={}{}",
                name_or_id.into(),
                verbose,
                scope
                    .map(|s| format!("&scope={}", s.into()))
                    .unwrap_or_default()
            )
        )
    }

    pub fn remove<ID: Into<String>>(&self, name_or_id: ID) -> String {
        self.base.at(format!("/networks/{}", name_or_id.into()))
    }

}

pub(crate) struct DockerEngineApiPathVolumes {
    base: DockerEngineApi
}

impl DockerEngineApiPathVolumes {

    pub fn create(&self) -> String {
        self.base.at("/volumes/create".into())
    }

    pub fn inspect<ID: Into<String>>(&self, name_or_id: ID) -> String {
        self.base.at(format!("/volumes/{}", name_or_id.into()))
    }

    pub fn list(&self) -> String {
        self.base.at("/volumes".into())
    }

    pub fn prune(&self) -> String {
        self.base.at("/volumes/prune".into())
    }

    pub fn remove<ID: Into<String>>(&self, name_or_id: ID, force: bool) -> String {
        self.base.at(format!("/volumes/{}?force={}", name_or_id.into(), force))
    }
}

/// Indicates URL type. Helps with choosing how to create and configure a hyper http client.
pub(crate) enum SchemedUrl {
    /// A url starting with http://
    Http(String),

    /// A url starting with https://
    Https(String),

    /// A reference starting with unix:// or simply /
    #[cfg(unix)]
    Unix(String)
}

impl ToString for SchemedUrl {
    fn to_string(&self) -> String {
        (match self {
            SchemedUrl::Http(url) => url,
            SchemedUrl::Https(url) => url,
            #[cfg(unix)]
            SchemedUrl::Unix(url) => url
        }).to_string()
    }
}

#[cfg(test)]
mod test_utils {
    use super::{DockerEngineApi, DockerEngineServer};

    pub(super) fn api_at(uri: &str) -> DockerEngineApi {
        let server = DockerEngineServer::new(uri)
            .unwrap();

        DockerEngineApi::new(server)
            .without_version()
    }
}

#[cfg(test)]
mod test_docker_engine_api {

    mod display {
        use super::super::test_utils::api_at;

        #[test]
        pub fn http() {
            let api = api_at("http://a:123");
            let actual = format!("{}", api);

            assert_eq!("http://a:123".to_string(), api.base);
            assert_eq!("Docker engine at http://a:123".to_string(), actual);
        }

        #[test]
        pub fn https() {
            let api = api_at("https://a:123");
            let actual = format!("{}", api);

            assert_eq!("https://a:123".to_string(), api.base);
            assert_eq!("Docker engine at https://a:123".to_string(), actual);
        }

        #[test]
        pub fn tcp() {
            let api = api_at("tcp://a:123");
            let actual = format!("{}", api);

            // Notice that the tcp: prefix got replaced with http:
            assert_eq!("http://a:123".to_string(), api.base);
            assert_eq!("Docker engine at tcp://a:123".to_string(), actual);
        }

        #[test]
        #[cfg(unix)]
        pub fn unix_explicit() {
            let api = api_at("unix:///some/path.sock");
            let actual = format!("{}", api);

            assert_eq!("unix://2f736f6d652f706174682e736f636b:0".to_string(), api.base);
            assert_eq!("Docker engine at /some/path.sock".to_string(), actual);
        }

        #[test]
        #[cfg(unix)]
        pub fn unix_implicit() {
            let api = api_at("/some/path.sock");
            let actual = format!("{}", api);

            assert_eq!("unix://2f736f6d652f706174682e736f636b:0".to_string(), api.base);
            assert_eq!("Docker engine at /some/path.sock".to_string(), actual);
        }
    }

    #[test]
    pub fn version() {
        use super::DockerEngineApi;

        let api = DockerEngineApi::without_server();

        assert_eq!("/version", api.version());
    }
}

#[cfg(test)]
mod test_docker_engine_api_path {

    mod ctor {
        use super::super::test_utils::api_at;

        #[test]
        pub fn from_http() {
            let api = api_at("http://a:123");
            let actual = api.containers().start("foo");
            let expected = "http://a:123/containers/foo/start";

            assert_eq!(expected, &actual);
        }

        #[test]
        pub fn from_https() {
            let api = api_at("https://a");
            let actual = api.containers().start("foo");
            let expected = "https://a/containers/foo/start";

            assert_eq!(expected, &actual);
        }

        #[test]
        #[cfg(unix)]
        pub fn from_unix_path() {
            let api = api_at("/var/run/docker.sock");
            let actual = api.containers().start("foo");
            let expected = "unix://2f7661722f72756e2f646f636b65722e736f636b:0/containers/foo/start";

            assert_eq!(expected, &actual);
        }

        #[test]
        #[cfg(unix)]
        pub fn from_unix_uri() {
            let api = api_at("unix:///var/run/docker.sock");
            let actual = api.containers().start("foo");
            let expected = "unix://2f7661722f72756e2f646f636b65722e736f636b:0/containers/foo/start";

            assert_eq!(expected, &actual);
        }
    }

    mod containers {
        use crate::requests::{ListContainersRequest, LogsArgs, RemoveContainerArgs, WaitCondition};
        use super::super::DockerEngineApi;

        #[test]
        pub fn create() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().create_exec("abc");

            assert_eq!("/containers/abc/exec", &actual);
        }

        #[test]
        pub fn create_named() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().create(Some(&"foo".to_string()));

            assert_eq!("/containers/create?name=foo", &actual);
        }

        #[test]
        pub fn create_no_name() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().create(None);

            assert_eq!("/containers/create", &actual);
        }

        #[test]
        pub fn files_changes() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().files("acme").changes();

            assert_eq!("/containers/acme/changes", &actual);
        }

        #[test]
        pub fn files_get() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().files("acme").get("/var/foo/bar");

            assert_eq!("/containers/acme/archive?path=/var/foo/bar", &actual);
        }

        #[test]
        pub fn inspect() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().inspect("foo", false);

            assert_eq!("/containers/foo/json?size=false", &actual);
        }

        #[test]
        pub fn inspect_with_size() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().inspect("foo", true);

            assert_eq!("/containers/foo/json?size=true", &actual);
        }

        #[test]
        pub fn kill() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap()
                .without_version();
            let actual = api.containers().kill("runaway", None)
                .unwrap();

            assert_eq!("http://a/containers/runaway/kill", actual);
        }

        #[test]
        pub fn kill_now() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap()
                .without_version();
            let actual = api.containers().kill("runaway", Some("SIGKILL".into()))
                .unwrap();

            assert_eq!("http://a/containers/runaway/kill?signal=SIGKILL", &actual);
        }

        #[test]
        pub fn list_all() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap()
                .without_version();
            let request = ListContainersRequest::default()
                .all(true);
            let actual = api.containers().list(request)
                .unwrap();

            assert_eq!("http://a/containers/json?all=true", &actual);
        }

        #[test]
        pub fn list_limit_10() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap()
                .without_version();
            let request = ListContainersRequest::default()
                .limit(10);
            let actual = api.containers().list(request)
                .unwrap();

            assert_eq!("http://a/containers/json?limit=10", &actual);
        }

        #[test]
        pub fn logs() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap()
                .without_version();
            let actual = api.containers().logs("chatty", LogsArgs::default())
                .unwrap();

            assert_eq!("http://a/containers/chatty/logs?stdout=true&stderr=true&timestamps=false", &actual);
        }

        #[test]
        #[cfg(not(windows))]  // Docker for Windows does not support pausing containers.
        pub fn pause() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().pause("x");

            assert_eq!("/containers/x/pause", &actual);
        }

        #[test]
        pub fn remove() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap()
                .without_version();
            let actual = api.containers().remove("x", RemoveContainerArgs::default())
                .unwrap();

            assert_eq!("http://a/containers/x", &actual);
        }

        #[test]
        pub fn rename() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().rename("abc", "xyz");

            assert_eq!("/containers/abc/rename?name=xyz", &actual);
        }

        #[test]
        pub fn remove_with_volumes() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap()
                .without_version();
            let args = RemoveContainerArgs::default()
                .remove_volumes(true);
            let actual = api.containers().remove("x", args)
                .unwrap();

            assert_eq!("http://a/containers/x?v=true", &actual);
        }

        #[test]
        pub fn start() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().start("bar");

            assert_eq!("/containers/bar/start", &actual);
        }

        #[test]
        pub fn stop() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().stop("qux");

            assert_eq!("/containers/qux/stop", &actual);
        }

        #[test]
        pub fn top() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap()
                .without_version();
            let actual = api.containers().top("b", Some("xyz".into()))
                .unwrap();

            assert_eq!("http://a/containers/b/top?ps_args=xyz", &actual);
        }

        #[test]
        #[cfg(not(windows))]  // Docker for Windows does not support pausing containers.
        pub fn unpause() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().unpause("x");

            assert_eq!("/containers/x/unpause", &actual);
        }

        #[test]
        pub fn wait_until_not_running() {
            let api = DockerEngineApi::without_server();
            let actual = api.containers().wait("w", WaitCondition::NotRunning);

            assert_eq!("/containers/w/wait?condition=not-running", &actual);
        }

    }

    mod images {
        use crate::imp::api::{DOCKER_ENGINE_VERSION_PATH, DockerEngineApi};
        use crate::requests::BuildImageRequest;

        #[test]
        pub fn build_typical() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap();

            let request = BuildImageRequest::default()
                .tag("foo:bar");

            let actual = api.images().build(request)
                .unwrap();

            assert_eq!(format!("http://a{}/build?t=foo%3Abar", DOCKER_ENGINE_VERSION_PATH), actual);
        }

        #[test]
        pub fn build_labeled() {
            let api = DockerEngineApi::with_server("http://a".into())
                .unwrap();

            let request = BuildImageRequest::default()
                .tag("qux")
                .label("w", "x")
                .label("y", "z");

            let actual = api.images().build(request)
                .unwrap();

            assert!(actual.starts_with(&format!("http://a{}/build?t=qux&labels=%7B%22", DOCKER_ENGINE_VERSION_PATH)));

            assert!(actual.contains('w'));
            assert!(actual.contains('x'));
            assert!(actual.contains('y'));
            assert!(actual.contains('z'));
        }

        #[test]
        pub fn json() {
            let api = DockerEngineApi::without_server();
            let actual = api.images().list();

            assert_eq!("/images/json", &actual);
        }

        #[test]
        pub fn tag() {
            let api = DockerEngineApi::without_server();
            let actual = api.images().tag("a:b", "c.com/def", "ghi");

            assert_eq!("/images/a:b/tag?repo=c.com/def&tag=ghi", &actual);
        }

        #[test]
        pub fn untag() {
            let api = DockerEngineApi::without_server();
            let actual = api.images().untag("b:c");

            assert_eq!("/images/b:c", &actual);
        }
    }

    mod exec {
        use crate::imp::api::DockerEngineApi;

        #[test]
        pub fn inspect() {
            let api = DockerEngineApi::without_server();
            let actual = api.exec().inspect("abc");

            assert_eq!("/exec/abc/json", &actual);
        }

        #[test]
        pub fn start() {
            let api = DockerEngineApi::without_server();
            let actual = api.exec().start("123");

            assert_eq!("/exec/123/start", &actual);
        }

    }

    mod networks {
        use crate::imp::api::DockerEngineApi;

        #[test]
        pub fn create() {
            let api = DockerEngineApi::without_server();
            let actual = api.networks().create();

            assert_eq!("/networks/create", &actual);
        }

        #[test]
        pub fn inspect_basic() {
            let api = DockerEngineApi::without_server();
            let actual = api.networks().inspect("abc", Option::<String>::None, false);

            assert_eq!("/networks/abc?verbose=false", &actual);
        }

        #[test]
        pub fn inspect_basic_verbose_in_scope() {
            let api = DockerEngineApi::without_server();
            let actual = api.networks().inspect("xyz", Some("qwerty"), true);

            assert_eq!("/networks/xyz?verbose=true&scope=qwerty", &actual);
        }

        #[test]
        pub fn remove() {
            let api = DockerEngineApi::without_server();
            let actual = api.networks().remove("a");

            assert_eq!("/networks/a", &actual);
        }

    }

    mod volumes {
        use crate::imp::api::DockerEngineApi;

        #[test]
        pub fn create() {
            let api = DockerEngineApi::without_server();
            let actual = api.volumes().create();

            assert_eq!("/volumes/create", &actual);
        }

        #[test]
        pub fn inspect() {
            let api = DockerEngineApi::without_server();
            let actual = api.volumes().inspect("foo");

            assert_eq!("/volumes/foo", &actual);
        }

        #[test]
        pub fn list() {
            let api = DockerEngineApi::without_server();
            let actual = api.volumes().list();

            assert_eq!("/volumes", &actual);
        }

        #[test]
        pub fn prune() {
            let api = DockerEngineApi::without_server();
            let actual = api.volumes().prune();

            assert_eq!("/volumes/prune", &actual);
        }

        #[test]
        pub fn remove() {
            let api = DockerEngineApi::without_server();
            let actual = api.volumes().remove("foo", true);

            assert_eq!("/volumes/foo?force=true", &actual);
        }

    }
}

#[cfg(test)]
mod test_docker_engine_server {
    use const_str::concat;
    use super::DOCKER_ENGINE_VERSION_PATH;
    use super::{DockerEngineApi, DockerEngineServer};

    // The default behavior is to include a reference to the Docker Engine version in the URL.
    //
    // Some tests suppress this behavior to simplify assertions about other URL components.
    #[test]
    fn adds_version_to_base_url() {
        let server = DockerEngineServer::new("http://foo")
            .unwrap();

        let api = DockerEngineApi::new(server);

        assert_eq!(concat!("http://foo", DOCKER_ENGINE_VERSION_PATH), api.base);
    }

}

#[cfg(test)]
mod test_sz_map {
    use std::collections::HashMap;
    use super::DockerEngineApiPathImages;

    #[test]
    fn empty() {
        let input: HashMap<String, i32> = HashMap::new();

        let actual = DockerEngineApiPathImages::sz_map(&input)
            .unwrap();

        assert_eq!(None, actual);
    }

    #[test]
    fn populated() {
        let input: HashMap<i32, String> = HashMap::from([
            (123, "a".into()),
            (456, "b".into())
        ]);

        let actual = DockerEngineApiPathImages::sz_map(&input)
            .unwrap();

        assert!(actual.is_some());
    }
}

#[cfg(test)]
mod test_sz_vec {
    use super::DockerEngineApiPathImages;

    #[test]
    fn empty() {
        let input: Vec<u16> = Vec::new();

        let actual = DockerEngineApiPathImages::sz_vec(&input)
            .unwrap();

        assert_eq!(None, actual);
    }

    #[test]
    fn populated() {
        let input: Vec<u128> = vec![12, 34];

        let actual = DockerEngineApiPathImages::sz_vec(&input)
            .unwrap();

        assert!(actual.is_some());
    }
}
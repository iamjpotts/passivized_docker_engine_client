use std::collections::HashMap;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Image/operation/ImageBuild
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BuildImageRequest {

    /// Path within the build context to the Dockerfile. This is ignored if
    /// remote is specified and points to an external Dockerfile.
    ///
    ///  Default: "Dockerfile"
    pub dockerfile: Option<String>,

    /// A name and optional tag to apply to the image in the name:tag format.
    /// If you omit the tag the default latest value is assumed. You can provide
    /// several name:tag parameters.
    pub tags: Vec<String>,

    pub extra_hosts: Option<String>,

    pub remote: Option<String>,

    pub quiet: Option<bool>,

    pub no_cache: Option<bool>,

    /// Images used for build cache resolution.
    pub cache_from: Vec<String>,

    /// Attempt to pull the image even if an older image exists locally.
    pub pull: Option<String>,

    /// Remove intermediate containers after a successful build.
    pub remove_intermediates: Option<bool>,

    /// Always remove intermediate containers, even upon failure.
    pub force_remove_intermediates: Option<bool>,

    pub memory_limit: Option<usize>,

    pub memory_and_swap: Option<isize>,

    pub cpu_shares: Option<usize>,

    pub cpu_set_cpus: Option<usize>,

    pub cpu_period: Option<u128>,

    pub cpu_quota: Option<u128>,

    pub build_args: HashMap<String, String>,

    pub shm_size_bytes: Option<usize>,

    pub squash: Option<bool>,

    pub labels: HashMap<String, String>,

    pub network_mode: Option<String>,

    pub platform: Option<String>,

    pub target: Option<String>,

    pub outputs: Option<String>
}

impl BuildImageRequest {

    pub fn dockerfile<V: Into<String>>(mut self, v: V) -> Self {
        self.dockerfile = Some(v.into());
        self
    }

    /// image:tag or just "image" where ":latest" is assumed.
    pub fn tag<V: Into<String>>(mut self, v: V) -> Self {
        self.tags.push(v.into());
        self
    }

    pub fn extra_hosts<V: Into<String>>(mut self, v: V) -> Self {
        self.extra_hosts = Some(v.into());
        self
    }

    pub fn remote<V: Into<String>>(mut self, v: V) -> Self {
        self.remote = Some(v.into());
        self
    }

    pub fn quiet(mut self, v: bool) -> Self {
        self.quiet = Some(v);
        self
    }

    pub fn no_cache(mut self, v: bool) -> Self {
        self.no_cache = Some(v);
        self
    }

    pub fn cache_from<V: Into<String>>(mut self, v: V) -> Self {
        self.cache_from.push(v.into());
        self
    }

    pub fn pull<V: Into<String>>(mut self, v: V) -> Self {
        self.pull = Some(v.into());
        self
    }

    pub fn remove_intermediates(mut self, v: bool) -> Self {
        self.remove_intermediates = Some(v);
        self
    }

    pub fn force_remove_intermediates(mut self, v: bool) -> Self {
        self.force_remove_intermediates = Some(v);
        self
    }

    pub fn memory_limit(mut self, v: usize) -> Self {
        self.memory_limit = Some(v);
        self
    }

    pub fn memory_and_swap(mut self, v: isize) -> Self {
        self.memory_and_swap = Some(v);
        self
    }

    pub fn cpu_shares(mut self, v: usize) -> Self {
        self.cpu_shares = Some(v);
        self
    }

    pub fn cpu_set_cpus(mut self, v: usize) -> Self {
        self.cpu_set_cpus = Some(v);
        self
    }

    pub fn cpu_period(mut self, v: u128) -> Self {
        self.cpu_period = Some(v);
        self
    }

    pub fn cpu_quota(mut self, v: u128) -> Self {
        self.cpu_quota = Some(v);
        self
    }

    pub fn build_arg<K, V>(mut self, k: K, v: V) -> Self
    where
        K: Into<String>,
        V: Into<String>
    {
        self.build_args.insert(k.into(), v.into());
        self
    }

    pub fn shm_size_bytes(mut self, v: usize) -> Self {
        self.shm_size_bytes = Some(v);
        self
    }

    pub fn squash(mut self, v: bool) -> Self {
        self.squash = Some(v);
        self
    }

    pub fn label<K, V>(mut self, k: K, v: V) -> Self
    where
        K: Into<String>,
        V: Into<String>
    {
        self.labels.insert(k.into(), v.into());
        self
    }

    pub fn network_mode<V: Into<String>>(mut self, v: V) -> Self {
        self.network_mode = Some(v.into());
        self
    }

    pub fn platform<V: Into<String>>(mut self, v: V) -> Self {
        self.platform = Some(v.into());
        self
    }

    pub fn target<V: Into<String>>(mut self, v: V) -> Self {
        self.target = Some(v.into());
        self
    }

    pub fn outputs<V: Into<String>>(mut self, v: V) -> Self {
        self.outputs = Some(v.into());
        self
    }

}
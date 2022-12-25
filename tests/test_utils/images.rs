
#[cfg(windows)]
pub const EXPECTED_PLATFORM: &str = "windows";

#[cfg(not(windows))]
pub const EXPECTED_PLATFORM: &str = "linux";

// Docker-in-Docker
pub mod dind {
    pub const IMAGE: &str = "docker";
    pub const TAG: &str = "20.10-dind";
    pub const PORT: u16 = 2376;
}

pub mod hello {
    pub const IMAGE: &str = "hello-world";
    pub const TAG: &str = "latest";
}

pub mod web {
    use std::collections::HashSet;

    pub const IMAGE: &str = "caddy";
    pub const TAG: &str = "2.6.1";

    #[cfg(windows)]
    pub const HASH: &str = "sha256:5f3406f4344708f83907899bede5acb43b539860c2da2c077a3c507a0a8efc4c";

    #[cfg(not(windows))]
    pub const HASH: &str = "sha256:181f13e1cc5fd90d0b4869e57689b5200589ca1029971fc7233082c04806a55b";

    // Some variance in size (+/- 1 MB) was observed even with a constant hash.
    pub const MIN_SIZE: i64 = 45000000;
    pub const PATH: &str = "caddy";
    pub const ENTRYPOINT: &str = "";

    #[cfg(windows)]
    pub const WORKING_DIR: &str = "";

    #[cfg(not(windows))]
    pub const WORKING_DIR: &str = "/srv";

    // A label the image is expected to have
    pub const LABEL_KEY: &str = "org.opencontainers.image.title";
    pub const LABEL_VALUE: &str = "Caddy";

    pub fn expected_driver() -> HashSet<String> {
        #[cfg(windows)]
        {
            HashSet::from(["windowsfilter".into()])
        }

        #[cfg(not(windows))]
        {
            HashSet::from([
                // Ubuntu
                "overlay2".into(),
                // Fedora
                "btrfs".into()])
        }
    }


    #[cfg(windows)]
    pub const EXPECTED_PROCESS: &str = "caddy.exe";

    #[cfg(not(windows))]
    pub const EXPECTED_PROCESS: &str = "caddy run";

    #[cfg(windows)]
    pub const EXPECTED_PROCESS_HEADING: &str = "Name";

    #[cfg(not(windows))]
    pub const EXPECTED_PROCESS_HEADING: &str = "CMD";

    #[cfg(windows)]
    pub const EXPECTED_ADDED_FILE: &str = "Files/config/caddy/autosave.json";

    #[cfg(not(windows))]
    pub const EXPECTED_ADDED_FILE: &str = "/config/caddy/autosave.json";

    pub fn args() -> Vec<String> {
        vec!["run".to_string(), "--config".into(), "/etc/caddy/Caddyfile".into(), "--adapter".into(), "caddyfile".into()]
    }

    pub fn cmd() -> Vec<String> {
        let mut result = args();
        result.insert(0, "caddy".into());

        result
    }

    pub fn entrypoint() -> Vec<String> {
        Vec::new()
    }

    pub fn exposed_ports() -> Vec<String> {
        vec![
            "80/tcp".into(),
            "443/tcp".into(),
            "443/udp".into(),
            "2019/tcp".into(),
        ]
    }

    pub const EXPECTED_ENV: &str = "CADDY_VERSION=v2.6.1";

    // Not a real image on Docker Hub; this is a name used by a test
    pub const TEST_TAG_PREFIX: &str = "temporary";
}

pub mod registry {
    pub const IMAGE: &str = "registry";
    pub const TAG: &str = "2.8";
}
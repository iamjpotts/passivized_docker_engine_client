use serde::Deserialize;

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerChanges
#[derive(Clone, Debug, Deserialize)]
pub struct FileSystemChange {
    #[serde(rename = "Path")]
    path: String,

    #[serde(rename = "Kind")]
    kind: i32,
}

impl FileSystemChange {

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn kind(&self) -> FileSystemChangeKind {
        FileSystemChangeKind::from_i32(self.kind)
    }

}

/// See https://docs.docker.com/engine/api/v1.41/#tag/Container/operation/ContainerChanges
#[derive(Debug, Eq, PartialEq)]
pub enum FileSystemChangeKind {
    Modified,
    Added,
    Deleted,
    Other(i32)
}

impl FileSystemChangeKind {

    pub fn from_i32(value: i32) -> FileSystemChangeKind {
        match value {
            0 => FileSystemChangeKind::Modified,
            1 => FileSystemChangeKind::Added,
            2 => FileSystemChangeKind::Deleted,
            _ => FileSystemChangeKind::Other(value)
        }
    }

}

#[cfg(test)]
mod test_file_system_change_kind {

    mod from_i32 {
        use crate::responses::FileSystemChangeKind;

        #[test]
        fn modified() {
            assert_eq!(FileSystemChangeKind::Modified, FileSystemChangeKind::from_i32(0))
        }

        #[test]
        fn other() {
            assert_eq!(FileSystemChangeKind::Other(123), FileSystemChangeKind::from_i32(123))
        }
    }

}
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) fn read_all_bytes<F: AsRef<Path>>(file_name: F) -> Result<Vec<u8>, std::io::Error> {
    let mut content: Vec<u8> = Vec::new();

    let mut f = File::open(file_name)?;
    f.read_to_end(&mut content)?;

    Ok(content)
}

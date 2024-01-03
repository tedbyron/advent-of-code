use std::{fs, path::PathBuf};

use anyhow::{anyhow, Error, Result};

pub fn input<F>(file_name: F) -> Result<String>
where
    PathBuf: From<F>,
{
    let path = PathBuf::from(file_name);

    fs::read_to_string(format!(
        "inputs/{}.txt",
        path.file_stem()
            .ok_or_else(|| anyhow!("missing file name"))?
            .to_string_lossy()
    ))
    .map_err(Error::from)
}

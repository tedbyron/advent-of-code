/// Read the input file for the current day.
#[macro_export]
macro_rules! read_input {
    () => {{
        std::fs::read_to_string(format!(
            "inputs/{}.txt",
            std::path::PathBuf::from(file!())
                .file_stem()
                .ok_or_else(|| anyhow::anyhow!("missing file stem"))?
                .to_string_lossy()
        ))
        .map_err(anyhow::Error::from)
    }};
}

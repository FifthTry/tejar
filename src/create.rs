pub struct Tejar {
    pub list: String,   // file
    pub files: Vec<u8>, // all file content
}

// TODO: Use struct with gzip
// pub struct InputFile {
//     path: camino::Utf8PathBuf,
//     content_type: String,
//     gzip: bool,
// }

pub fn create(
    root: &camino::Utf8Path,
    files: &[(camino::Utf8PathBuf, String)],
) -> Result<Tejar, crate::error::CreateError> {
    let mut list_content = "".to_string();
    let mut files_content = vec![];

    for (file, content_type) in files.iter() {
        let path = root.join(file);
        let mut content = std::fs::read(path)?;
        // TODO: use compression?
        //
        list_content
            .push_str(format!("{}|{}|{}\n", file.as_str(), content_type, content.len()).as_str());

        files_content.append(&mut content);
    }

    Ok(Tejar {
        list: list_content,
        files: files_content,
    })
}

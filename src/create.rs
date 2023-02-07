pub struct Tejar {
    list: String,   // file
    files: Vec<u8>, // all file content
}

pub fn create(
    root: &std::path::Path,
    files: Vec<&std::path::Path>,
) -> Result<Tejar, crate::error::CreateError> {
    // provided path should be directory or file, with user permission
    // it will read all the files recursively and create two file list.json and path/<list-path>.tejar
    // should we use compression?
    // write both the in the current directly
    Ok(Tejar {
        list: "".to_string(),
        files: vec![],
    })
}

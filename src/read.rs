pub struct TejarRecord {
    file_size: u32,
    file_name: String,
    content_type: String,
}

pub struct Reader {
    list: Vec<TejarRecord>, // or hashmap opaque
}

pub struct FileInfo {
    content_type: String,
    offset: u32,
    file_size: u32,
}

impl Reader {
    pub fn get_file_info(
        &self,
        path: &std::path::Path,
    ) -> Result<FileInfo, crate::error::ReadError> {
        Ok(FileInfo {
            content_type: "".to_string(),
            offset: 0,
            file_size: 0,
        })
    }

    pub fn get_content(&self, files_content: &str) -> Result<Vec<u8>, crate::error::ReadError> {
        Ok(vec![])
    }
}

pub fn reader(list_content: &str) -> Result<Reader, crate::error::ReadError> {
    Ok(Reader { list: vec![] })
}

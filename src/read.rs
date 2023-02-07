pub struct TejarRecord {
    file_name: String,
    offset: u32,
    file_size: u32,
    content_type: String,
}

pub struct Reader {
    list: Vec<TejarRecord>, // or hashmap opaque
}

pub struct FileInfo {
    pub content_type: String,
    pub offset: u32,
    pub file_size: u32,
}

impl Reader {
    pub fn get_file_info(
        &self,
        path: &std::path::Path,
    ) -> Result<FileInfo, crate::error::ReadError> {
        let file_record = self
            .list
            .iter()
            .find(|r| std::path::Path::new(&r.file_name).eq(path))
            .ok_or(crate::error::ReadError::NotFound(format!(
                "file not found: {:?}",
                path
            )))?;

        Ok(FileInfo {
            content_type: file_record.content_type.to_string(),
            offset: file_record.offset,
            file_size: file_record.file_size,
        })
    }
}

fn parse_list(list_content: &str) -> Result<Vec<TejarRecord>, crate::error::ReadError> {
    // TODO: remove trim, needed because one extra line is getting appended in list content
    let lines = list_content.trim().split('\n');
    let mut records = Vec::new();
    let mut offset = 0;
    for (index, line) in lines.enumerate() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 3 {
            let record = TejarRecord {
                file_name: parts[0].to_string(),
                offset,
                content_type: parts[1].to_string(),
                file_size: parts[2].parse::<u32>().map_err(|e| {
                    crate::error::ReadError::ParseError {
                        line: index + 1,
                        message: e.to_string(),
                    }
                })?,
            };
            offset = offset + record.file_size;
            records.push(record);
        } else {
            return Err(crate::error::ReadError::ParseError {
                line: index + 1,
                message: line.to_string(),
            });
        }
    }
    Ok(records)
}

pub fn reader(list_content: &str) -> Result<Reader, crate::error::ReadError> {
    Ok(Reader {
        list: parse_list(list_content)?,
    })
}

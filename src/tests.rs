#[test]
fn test1() {
    fn current_dir() -> camino::Utf8PathBuf {
        let current_dir = std::env::current_dir().unwrap();
        camino::Utf8PathBuf::from_path_buf(current_dir).unwrap()
    }

    fn walkdir_util(root: &camino::Utf8Path) -> Vec<(camino::Utf8PathBuf, String)> {
        walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(|e| {
                if !e.as_ref().unwrap().file_type().is_dir() {
                    Some(e)
                } else {
                    None
                }
            })
            .map(|e| {
                camino::Utf8PathBuf::from(e.as_ref().unwrap().path().as_os_str().to_str().unwrap())
            })
            .map(|x| (x.into(), "todo!".to_string()))
            .collect::<Vec<(camino::Utf8PathBuf, String)>>()
    }

    let root = current_dir().join("tejar-tests/1");
    let files: _ = walkdir_util(root.as_path())
        .into_iter()
        .map(|(p, d)| (p.strip_prefix(&root).unwrap().to_path_buf(), d))
        .collect::<Vec<_>>();

    let t = crate::create::create(&root, files.as_slice()).unwrap();

    println!("List Content");
    println!("{}", t.list);

    // TODO: Read the content of individual file and match the content from reader output
    let reader = crate::read::reader(&t.list).unwrap();
    let file_info = reader
        .get_file_info(std::path::Path::new("foo/index.html"))
        .unwrap();

    let start = file_info.offset as usize;
    let end = start + file_info.file_size as usize;

    let content = String::from_utf8(t.files.as_slice()[start..end].to_vec());
    println!(
        "Offset: {}, Size: {}, Content: {}",
        file_info.offset,
        file_info.file_size,
        content.unwrap(),
    );

    println!("Files Content");
    println!("{}", String::from_utf8(t.files).unwrap());
}

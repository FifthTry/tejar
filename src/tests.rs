#[test]
fn test1() {
    // TODO: Test Contain so many unwraps

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

    println!("Files Content");
    println!("{}", String::from_utf8(t.files.clone()).unwrap());

    // TODO: Read the content of individual file and match the content from reader output
    let reader = crate::read::reader(&t.list).unwrap();
    for (file, _) in files.iter() {
        let file_path = root.join(file);
        let original_content = String::from_utf8(std::fs::read(file_path).unwrap()).unwrap();

        let file_info = reader.get_file_info(file.as_std_path()).unwrap();
        let content = crate::read::get_content(
            file_info.offset as usize,
            file_info.file_size as usize,
            t.files.as_slice(),
        )
        .unwrap();

        assert_eq!(original_content, content);
    }
}

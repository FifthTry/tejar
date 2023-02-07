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
    println!("Files Content");
    println!("{}", String::from_utf8(t.files).unwrap());
}

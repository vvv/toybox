use walkdir::WalkDir;

fn walk(root: &str) -> Result<(), ()> {
    let mut prev = None;
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .enumerate()
        .take_while(|(line, entry)| {
            let path = entry.path();
            if let Some(prev) = prev.replace(path.to_path_buf()) {
                if prev == path {
                    eprintln!("**ERROR** The same input line appears twice in a row. Aborting.");
                    eprintln!("[Line {}] {:?}", line, path);
                    return false;
                }
            }
            true
        })
        .for_each(|(_, entry)| println!("{}", entry.path().display()));
    Ok(())
}

fn main() {
    let root = {
        let mut args = std::env::args();
        let _prog = args.next().unwrap();
        args.next().unwrap_or_else(|| ".".to_owned())
    };
    std::process::exit(walk(&root).map_or(1, |_| 0))
}

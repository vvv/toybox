use std::path::PathBuf;
use walkdir::WalkDir;

fn walk(root: &str) -> Result<(), ()> {
    let mut prev = PathBuf::new();
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .take_while(|e| {
            let path = e.path();
            if path == prev.as_path() {
                eprintln!("**ERROR** The same path appears two times in a row. Aborting.");
                eprintln!("{:?}", path);
                return false;
            }
            prev = path.to_path_buf();
            true
        })
        .filter(|e| !e.file_type().is_dir())
        .for_each(|e| println!("{}", e.path().display()));
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

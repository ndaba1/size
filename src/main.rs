use std::path::{Path, PathBuf};

fn main() -> Result<(), std::io::Error> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        panic!("Please pass in a single argument");
    }

    let current = std::env::current_dir()?;
    let target = Path::join(&current.as_path(), &args[1]);
    let size = read_size(&target)?;

    let size = size / 1_000_000;
    println!("{} is of size {}MB", target.display(), size);

    Ok(())
}


fn read_size(target: &PathBuf) -> Result<u64, std::io::Error> {
    let mut size = 0 as u64;
    if target.is_file() {
        let meta = std::fs::metadata(&target)?;
        size = meta.len();
    } else if target.is_dir() {
        let res = std::fs::read_dir(target)?;
        for r in res.into_iter() {
            let p = r?;
            size += read_size(&p.path())?
        }
        
    }

    Ok(size)
}
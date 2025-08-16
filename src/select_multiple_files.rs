//Select multiple files

use std::{fs, io, path::PathBuf};

pub fn read_multiple_files(paths : &[PathBuf])-> io::Result<Vec<String>>{
    paths.iter().map(|p| fs::read_to_string(p)).collect()
}

//multiple file Logic testing

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_read_multiple_files(){
        let dir = env::temp_dir().join("rust test_multi");
        std::fs::create_dir_all(&dir).unwrap();

        let f1 = dir.join("a.txt");
        let f2 = dir.join("b.txt");

        fs::write(&f1, "file1").unwrap();
        fs::write(&f2, "file2").unwrap();

        let out = read_multiple_files(&[f1.clone() , f2.clone()]).unwrap();
        assert_eq!(out , vec!["file1".to_string(), "file2".to_string()]);

        fs::remove_dir_all(&dir).unwrap()
    }
}
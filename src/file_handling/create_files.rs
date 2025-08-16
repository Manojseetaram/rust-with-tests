use std::{fs::{ File}, io, path::Path};


//create file
pub fn create_file<P : AsRef<Path>>(path : P)-> io::Result<File>{
   File::create(path)
}
//create file test cases 
#[cfg(test)]

mod tests{
    use super::*;
    use std::env;
    use std::fs;

#[test]
fn test_crate_file(){
    let path = env::temp_dir().join("test_crete_file.txt");
    let _file = create_file(&path).unwrap();
    assert!(path.exists() && path.is_file());
    fs::remove_file(path).unwrap();
}
}
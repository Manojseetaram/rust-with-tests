use std::{fs, io, path::Path};



pub fn delete_file<P : AsRef<Path>>(path : P)-> io::Result<()>{
      fs::remove_file(path)
}
pub fn delete_folder<P : AsRef<Path>>(path : P)-> io::Result<()>{
    fs::remove_dir_all(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
   fn delete_file_and_folder(){
    let dir = env::temp_dir().join("rust_delete_file_and_folder");
    fs::create_dir_all(&dir).unwrap();

    let file = dir.join("test.txt");
    fs::write(&file, "temp").unwrap();  
    assert!(file.exists()); 

    delete_file(&file).unwrap();
    assert!(!file.exists());
    delete_folder(&dir).unwrap();
    assert!(!dir.exists())

   }
}
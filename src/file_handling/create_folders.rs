use std::{fs, io, path::Path};


//when we new directoty 
pub fn create_folder<P:AsRef<Path>>(path : P)-> io::Result<()>{
    fs::create_dir_all(path)
}

//create new file tesing 

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;
   #[test]
    fn test_create_folder(){
        let path = env::temp_dir().join("rust_test_foledr");
        create_folder(&path).unwrap();
        assert!(path.exists() && path.is_dir());
        fs::remove_dir_all(&path).unwrap()
    }
}
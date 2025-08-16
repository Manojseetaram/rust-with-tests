use std::{fs, io, path::Path};



pub fn delete_file<P : AsRef<Path>>(path : P)-> io::Result<()>{
      fs::remove_dir(path)
}
pub fn delete_folder<P : AsRef<Path>>(path : P)-> io::Result<()>{
    fs::remove_dir_all(path)
}
use std::{fs, io, path::Path};

//read the file

pub fn read_file_to_string<P : AsRef<Path>>(path : P)-> io::Result<String>{
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ env, time::{SystemTime, UNIX_EPOCH}};

    #[test]

 fn read_file(){
    let name = format!("read_{}.txt",now_ms());
    let path =env::temp_dir().join(name);
    fs::write(&path, "hello").unwrap();
   let s = read_file_to_string(&path).unwrap();
   assert_eq!(s , "hello");
   fs::remove_file(path).unwrap();

}
fn now_ms()-> u128{
       SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}
}
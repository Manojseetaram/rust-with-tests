use std::{ io, path::{ Path}};

//Save state export data we use this logic 
pub fn write_files<P : AsRef<Path>>(path: P , contents : &str)-> io::Result<()>{
  std::fs::write(path, contents)
}
pub fn write_file_atomic<P: AsRef<Path>>(path : P , contents : &str)-> io::Result<()>{
   use std::io::Write;
   let final_path = path.as_ref().to_path_buf();
   let tmp = final_path.with_extension("tmp_write");
   let mut f =std::fs::File::create(&tmp)?;
    f.write_all(contents.as_bytes())?;
    f.sync_all()?;
    std::fs::rename(tmp, final_path)?;
    Ok(())

}

#[cfg(test)]

mod tests{
    use super::*;
    use std::{env,fs, time::{SystemTime, UNIX_EPOCH}};



    #[test]


    fn write_and_atomic_work(){
        let base = env::temp_dir();
        let p1 = base.join(format!("write_{}.txt", now_ms()));
        write_files(&p1, "abc").unwrap();
        assert_eq!(fs::read_to_string(&p1).unwrap(),"abc");
        fs::remove_file(&p1).unwrap();

        let p2 = base.join(format!("write_atomic_{}.txt", now_ms()));
        write_file_atomic(&p2, "xyz").unwrap();
        assert_eq!(fs::read_to_string(&p2).unwrap(), "xyz");
        fs::remove_file(&p2).unwrap();

    }
     fn now_ms() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
}
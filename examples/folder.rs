
use std::{fs, path::Path};

fn main(){
    let dir = Path::new(".");
    if dir.is_dir() {
        let d=fs::read_dir(dir).unwrap().into_iter().filter(|x: &Result<fs::DirEntry, std::io::Error>|{
            match x{
                Ok(i)=>{
                    i.path().is_file()
                },
                Err(_)=>
                    false
            }
        }).map(|x|{
            x.unwrap().path().file_name().unwrap().to_owned()
        }).collect::<Vec<std::ffi::OsString>>();
        // for entry in fs::read_dir(dir).unwrap() {
        //     let entry = entry.unwrap();
        //     let path = entry.path();
        //     if path.is_file(){
        //         println!("{:?}",path.file_name());
        //     }

        // }
        println!("{:?}",d);
    }

}
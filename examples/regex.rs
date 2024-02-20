
use std::{fs::*, io::Read};
use regex::Regex;


fn main(){
    let f = File::open("aa.js");
    match &f {
        Err(e) => {
            println!("{}", format!("{e}"));
            return;
        }
        _ => (),
    };
    let mut buffer = String::new();
    // read the whole file
    f.unwrap()
        .read_to_string(&mut buffer)
        .expect("can not read file content");
    let re=Regex::new(r"^let\b").unwrap();
    let mut res=Vec::<(usize,usize,&str)>::new();
    for (index, line) in buffer.lines().enumerate() {
        // println!("{index}:{row}");
        match re.find(&line){
            Some(c)=>{
                res.push((index,c.range().start,line));
            },
            _=>()
        }
    }
    // let res=test.lines().into_iter().map(|x| (re.find(x).unwrap().range().start,re.find(x).unwrap().as_str())).collect::<Vec<(usize,&str)>>();
    println!("{:?}",res);
}



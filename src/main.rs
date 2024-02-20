use clap::Parser;
use regex::Regex;
use std::{collections::HashMap, fs, fs::*, io::Read, path::Path, vec};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

fn main() {
    let opts: Opts = Opts::parse();
    let server = Server::new(&opts);
    server.find().show();
}

fn print_syntect(s: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

/// find the current content and return row number
#[derive(Parser, Debug)]
struct Opts {
    ///the text you want to find
    content: String,
    ///the file which you want to searching
    file: String,
}

///search server
struct Server<'a> {
    opt: &'a Opts,
}

impl<'a> Server<'a> {
    ///return a Server object created by Opts you provided 
    pub fn new(opt: &Opts) -> Server {
        Server { opt }
    }

    ///return the result of searching 
    pub fn find(&self) -> SearchResult {
        //find current content in each source file
        let mut hashmap = HashMap::new();
        let re = Regex::new(&self.opt.content);
        for buffer in self.get_source() {
            let mut res = Vec::new();
            if let Ok(re) = &re {
                for (index, line) in buffer.1.lines().enumerate() {
                    match re.find(&line) {
                        Some(c) => {
                            let d = format!("{:2}:{:2}  {}\n", index+1, c.range().start, line);
                            res.push(d);
                        }
                        _ => (),
                    }
                }
            }
            hashmap.insert(buffer.0, res);
        }
        SearchResult {
            data: Some(hashmap),
        }
    }

    ///private method. Return a list of tuples constructed by the names and contents of files which match
    fn get_source(&self) -> Vec<(String, String)> {
        //find file by regex or file name
        if let Ok(re) = Regex::new(&self.opt.file) {
            let file_names = fs::read_dir(Path::new("."))
                .unwrap()
                .into_iter()
                .filter(|x: &Result<fs::DirEntry, std::io::Error>| match x {
                    Ok(i) => i.path().is_file(),
                    Err(_) => false,
                })
                .map(|x| x.unwrap().path().file_name().unwrap().to_owned())
                .collect::<Vec<std::ffi::OsString>>();
            file_names
                .iter()
                .filter(|x| re.is_match(x.to_str().unwrap()))
                .map(|x| {
                    let name = x.to_str().unwrap().to_owned();
                    let mut f = File::open(&name).expect("can not open file");
                    let mut buffer = String::new();
                    f.read_to_string(&mut buffer)
                        .expect("can not read file content");
                    (name, buffer)
                })
                .collect::<Vec<(String, String)>>()
        }else{
            vec![("".to_string(),"".to_string())]
        }
    }
}

#[derive(Debug, PartialEq)]
struct SearchResult {
    data: Option<HashMap<String, Vec<String>>>,
}

impl SearchResult {
    //print the result of finding to console
    fn show(&self) {
        let mut res = "".to_string();
        if let Some(i) = &self.data {
            for (k, v) in i.into_iter() {
                res += format!("{}\n", k).as_ref();
                res += v.join("").as_ref();
            }
        }
        print_syntect(&res);
    }
}


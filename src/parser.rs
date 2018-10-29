//lalrpop_mod!(pub socool);
//extern crate colored;
//use colored::*;
//use std::fs::File;
//use std::io::prelude::*;
//use std::collections::HashMap;
//use std::sync::{Arc, Mutex};
//use std::cmp;
//use std::io::BufReader;
//use crate::ast::*;
//
#[derive(Clone, PartialEq, Debug)]
pub struct Init {
    pub f: f32,
    pub l: f32,
    pub g: f32,
    pub p: f32,
}
//
//pub type ParseTable = HashMap<String, Op>;
//
//#[derive(Clone, PartialEq, Debug)]
//pub struct ParsedComposition {
//    pub init: Init,
//    pub table: ParseTable,
//}
//
//pub fn parse_file(filename: &String) -> ParsedComposition {
//    let mut table = HashMap::new();
//    let f = File::open(filename);
//    let mut composition = String::new();
//
//    match f {
//        Ok(f) => {
//            let file = BufReader::new(&f);
//            for line in file.lines() {
//                let l = line.unwrap();
//                let copy_l = l.trim_left();
//                if copy_l.starts_with("--") {
//                    composition.push_str("\n");
//                } else {
//                    composition.push_str("\n");
//                    composition.push_str(&l);
//                }
//            }
//        },
//        _ => {
//            println!("{} {}\n", "\n        File not found:".red().bold(), filename.red().bold());
//            panic!("File not found");
//        }
//    }
//
//    let init = socool::SoCoolParser::new()
//        .parse(&mut table, &composition);
//
//    match init.clone() {
//        Ok(init) => ParsedComposition { init, table },
//        Err(error) => {
//            let start_offset = 150;
//            let end_offset = 50;
//            let location = Arc::new(Mutex::new(Vec::new()));
//            error.map_location(|l| location.lock().unwrap().push(l));
//            let start = location.lock().unwrap()[0];
//            let end =  location.lock().unwrap()[1];
//            let cmp_len = &composition.len();
//            let feed_start = cmp::max(0, start as isize - start_offset) as usize;
//            let feed_end = cmp::min(end + end_offset, *cmp_len);
//
//            let mut lines = 1;
//            let mut n_c = 0;
//            for c in composition.clone().chars() {
//                n_c += 1;
//                if n_c > start { break; }
//
//                if c == '\n' {
//                    lines += 1
//                }
//            }
//            println!("{}{}",
//                &composition[feed_start..start].yellow(),
//                &composition[start..feed_end].red(),
//            );
//
//
//            println!("
//            {}
//            errors at line {}
//            {}
//            ",
//             "working".yellow().underline(),
//             lines.to_string().red().bold(),
//             "broken".red().underline(),
//            );
//
//            panic!("Unexpected Token")
//        }
//    }
//}
//
//

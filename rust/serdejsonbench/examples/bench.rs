use serdejsonbench::JsonIterator;
use serdejsonbench::{iter_json_array, read_from_file2, Json};
use std::fs::File;
use std::{io::Write, time::Instant};

use std::io::BufReader;

fn parsev3() {
    let iter = read_from_file2(r#"../../json/256MB.json"#).unwrap();
    let mut count = 0;
    for json in iter {
        assert_eq!("FULL", json.delta_mode);
        count = count + 1;
    }
    assert_eq!(68495, count);
}

fn parsev2() {
    let reader = BufReader::with_capacity(8192, File::open(r#"../../json/256MB.json"#).unwrap());
    let iter = iter_json_array(reader);
    let mut count = 0;
    for json in iter {
        let json: Json = json.unwrap();
        assert_eq!("FULL", json.delta_mode);
        count = count + 1;
    }
    assert_eq!(68495, count);
}

fn parse() {
    let iter = JsonIterator::new(r#"../../json/256MB.json"#.into());
    let mut count = 0;
    for json in iter {
        let json = json.unwrap();
        assert_eq!("FULL", json.delta_mode);
        count = count + 1;
    }
    assert_eq!(68495, count);
}

enum Method {
    V1,
    V2,
    V3,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() > 2 && args[0] == "--console" && args[1] == "--times" {
        let now = Instant::now();
        let times: u32 = args[2].as_str().parse().unwrap();

        let method = if args.len() > 4 && args[3] == "--method" {
            match args[4].as_str() {
                "v2" => {
                    println!("v2 version");
                    Method::V2
                }
                "v3" => {
                    println!("v3 version");
                    Method::V3
                }
                _ => panic!("unexpected version"),
            }
        } else {
            println!("v1 version");
            Method::V1
        };

        let parse = match method {
            Method::V1 => parse,
            Method::V2 => parsev2,
            Method::V3 => parsev3,
        };

        for _ in 0..times {
            print!(".");
            std::io::stdout().flush().unwrap();
            parse();
        }

        let elapsed = now.elapsed().as_millis();
        let avg = elapsed / u128::from(times);
        println!("done in {elapsed}ms avg {avg}ms");
    } else {
        panic!("bad arguments {args:?}")
    }

    println!("done");
}

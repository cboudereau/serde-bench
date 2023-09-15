use serdejsonbench::JsonIterator;
use serdejsonbench::{iter_json_array, Json};
use std::fs::File;
use std::{io::Write, time::Instant};

use std::io::BufReader;

fn parsev2() {
    println!("v2 version");
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
    println!("v1 version");
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
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() > 2 && args[0] == "--console" && args[1] == "--times" {
        let now = Instant::now();
        let times: u32 = args[2].as_str().parse().unwrap();

        let method = if args.len() > 4 && args[3] == "--method" && args[4] == "v2" {
            Method::V2
        } else {
            Method::V1
        };

        let parse = match method {
            Method::V1 => parse,
            Method::V2 => parsev2,
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

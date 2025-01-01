use std::{env, thread, time::Duration};

use include_dir::{include_dir, Dir};

const FIREWORKS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/fireworks");
const FIREPLACE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/fireplace");
const RICKROLL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/rick_ascii");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--help")) {
        println!();
        println!("Play text art animations in the terminal\n");
        println!("Usage: firework-rs [folder] [loops]");
        println!("\t[folder]\tFolder containing text art frames: fireworks | fireplace | rick_ascii (default: fireworks)");
        println!("\t[loops]\t\tNumber of times to loop the animation or use -1 to loop until the user terminates the program (default: 20)");
        println!();
        return;
    }
    let folder_name = args
        .get(1)
        .map_or("fireworks".to_string(), |s| s.to_string());
    let loops: i32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);

    let folder = match folder_name.as_str() {
        "fireworks" => FIREWORKS_DIR,
        "fireplace" => FIREPLACE_DIR,
        "rick_ascii" => RICKROLL_DIR,
        _ => panic!("folder {folder_name} not available"),
    };

    let mut frames = vec![];
    let mut file_found = true;
    let mut file_number = 0;
    while file_found {
        if let Some(f) = folder.get_file(format!("{file_number}.txt")) {
            frames.push(f.contents_utf8().unwrap_or_else(|| panic!("unable to read file {file_number}.txt from folder {folder_name} into utf8 string")));
            file_number += 1;
        } else {
            file_found = false;
        }
    }

    let mut i = 0;
    let mut first = true;
    let num_lines = frames[0].lines().count();
    let backspace_adjust = "\x1b[A".repeat(num_lines + 1);

    while i < loops || loops == -1 {
        for frame in frames.iter() {
            if !first {
                println!("{backspace_adjust}");
            }
            println!("{frame}");
            first = false;
            thread::sleep(Duration::from_millis(50));
        }
        i += 1;
    }
}

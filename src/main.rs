mod basm;
mod errors;

use std::env;
use std::fs;

use errors::*;

use basm::CompilationData;

fn use_bit(bitfield: &mut u32, pos: u32) {
    *bitfield |= 1u32 << pos;
}

fn find_el(vector: &Vec<String>, bitfield: &mut u32, target: &str) -> (bool, usize) {
    let (found, pos) = vector.iter().position(|x| x.as_str() == target).map_or((false, 0), |pos| (true, pos));
    if found {
        use_bit(bitfield, pos as u32);
    }
    (found, pos)
}



fn main() {
    let mut data = CompilationData::default();

    let args: Vec<String> = env::args().collect();
    let mut used: u32 = 0b11; // checks which terminal arguments are used

    let (found, pos) = find_el(&args, &mut used, "-o");
    if found {
        use_bit(&mut used, pos as u32);

        if args.len() == pos + 1 {
            eprintln!("Error: Could not find output filename after -o");
        } else {
            use_bit(&mut used, (pos + 1) as u32);
            data.outname = args[pos + 1].clone();
        }
    }

    let warnings;
    {
        use basm::WarningType::*;

        warnings = [
            (Op, "-Wop", "-Wno-op"),
            (Uid, "-Wuid", "-Wuid-op"),
            (Parent, "-Wparent", "-Wno-parent"),
            (Next, "-Wnext", "-Wno-next"),
            (In, "-Win", "-Wno-in"),
            (Field, "-Wfield", "-Wno-field"),
            (Mut, "-Wmut", "-Wno-mut"),
            (Shadow, "-Wshadow", "-Wno-shadow")
        ];
    }

    for (warning, name, no_name) in warnings {
        let (found, _) = find_el(&args, &mut used, name);
        if found { data.warn.insert(warning); }

        let (found, _) = find_el(&args, &mut used, no_name);
        if found { data.no_warn.insert(warning); }
    }



    let (reverse_found, _) = find_el(&args, &mut used, "--reverse");
    if data.verbose {
        for (warning, name, no_name) in warnings {
            if data.warn.contains(&warning) {
                if reverse_found { throw_warning(&format!("{} is ignored when using --reverse", name)); }
                if data.wall { throw_warning(&format!("{} is redundant when used with -Wall", name)); }
                if data.no_warn.contains(&warning) {
                    throw_warning(&format!("{} and {} are redundant when used together", name, no_name));
                }  
            }
        }
    }

    

    if used & (1u32 << (args.len() - 1)) == 0 {
        let filename = args.last().unwrap();
        match fs::read_to_string(filename) {
            Ok(value) => {
                data.source = value;
                data.outname = (*filename).clone();
            },
            Err(e) => {
                throw_error(format!("File {}, {}", filename, e));
            }
        };
    }
}
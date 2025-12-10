mod basm;
mod errors;

use std::env;
use std::fs;

use errors::error;
use errors::warn;



#[derive(Default)]
struct CompilationData {
    outname: String,
    source: String,
    verbose: bool,
    log: bool,
    wop: bool,
    wuid: bool,
    wparent: bool,
    wnext: bool,
    win: bool,
    wfield: bool,
    wmut: bool,
    wshadow: bool,
    wall: bool,
    werror: bool,
    wno_op: bool,
    wno_uid: bool,
    wno_parent: bool,
    wno_next: bool,
    wno_in: bool,
    wno_field: bool,
    wno_mut: bool,
    wno_shadow: bool,
    stdout: bool,
    reverse: bool
}



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

    let mut used: u32 = 0b11;

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

    let flags = [
        (&mut data.verbose, "--verbose"),
        (&mut data.stdout, "--stdout"),
        (&mut data.reverse, "--reverse"),
        (&mut data.log, "--log"),
        (&mut data.wop, "-Wop"),
        (&mut data.wuid, "-Wuid"),
        (&mut data.wparent, "-Wparent"),
        (&mut data.wnext, "-Wnext"),
        (&mut data.win, "-Win"),
        (&mut data.wfield, "-Wfield"),
        (&mut data.wmut, "-Wmut"),
        (&mut data.wshadow, "-Wshadow"),
        (&mut data.wall, "-Wall"),
        (&mut data.werror, "-Werror"),
        (&mut data.wno_op, "-Wno-op"),
        (&mut data.wno_uid, "-Wno-uid"),
        (&mut data.wno_parent, "-Wno-parent"),
        (&mut data.wno_next, "-Wno-next"),
        (&mut data.wno_in, "-Wno-in"),
        (&mut data.wno_field, "-Wno-field"),
        (&mut data.wno_mut, "-Wno-mut"),
        (&mut data.wno_shadow, "-Wno_shadow")
    ];

    for (flag, name) in flags {
        let (found, _) = find_el(&args, &mut used, name);
        if found {
            *flag = true;
        }
    }




    let (found, _) = find_el(&args, &mut used, "--reverse");
    if data.verbose {
        let w_flags = [
            (data.wop, "-Wop"),
            (data.wuid, "-Wuid"),
            (data.wparent, "-Wparent"),
            (data.wnext, "-Wnext"),
            (data.win, "-Win"),
            (data.wfield, "-Wfield"),
            (data.wmut, "-Wmut"),
            (data.wall, "-Wall"),
            (data.werror, "-Werror"),
            (data.wshadow, "-Wshadow")
        ];

        if found {
            for (flag_set, flag_name) in w_flags {
                if flag_set {
                    warn(&format!("{} is ignored when using --reverse", flag_name));
                }
            }
        }
        if data.wall {
            for (flag_set, flag_name) in w_flags {
                if flag_set {
                    warn(&format!("{} is redundant when used with -Wall", flag_name));
                }
            }
        }

        let wno_flags = [
            (data.wno_op, "-Wno-op"),
            (data.wno_uid, "-Wno-uid"),
            (data.wno_parent, "-Wno-parent"),
            (data.wno_next, "-Wno-next"),
            (data.wno_in, "-Wno-in"),
            (data.wno_field, "-Wno-field"),
            (data.wno_mut, "-Wno-mut"),
            (data.wno_shadow, "-Wno_shadow")
        ];

        for ((w_flag, w_name), (wno_flag, wno_name)) in w_flags.iter().zip(wno_flags.iter()) {
            if *w_flag && *wno_flag {
                warn(&format!("{} and {} are redundant when used together", w_name, wno_name));
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
                error(format!("File {}, {}", filename, e));
            }
        };
    }
}
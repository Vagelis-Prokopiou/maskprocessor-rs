// https://github.com/hashcat/maskprocessor/blob/master/src/mp.c
use maskprocessor_rs::*;
use clap::{Arg, App};
use std::process::exit;
use std::fs::File;
use std::io::Write;

fn main() {
    let mask = "mask";
    let combinations = "combinations";
    let custom_charset_value_name = "charset";
    let file = "file";

    /*
    let increment = "increment";
    let hex_charset = "hex-charset";
    let seq_max = "seq-max";
    let start_at = "start-at";
    let stop_at = "stop-at";
    let occurrence_max = "occurrence-max";

    let arg_increment = Arg::with_name(increment)
        .conflicts_with(start_at)
        .conflicts_with(stop_at)
        .short("i")
        .long(increment)
        .takes_value(true)
        .number_of_values(2)
        .value_names(&["num", "num"])
        .help("Enable increment mode");
    let arg_hex_charset = Arg::with_name(hex_charset)
        .long(hex_charset)
        .help("Assume charset is given in hex");
    let arg_seq_max = Arg::with_name(seq_max)
        .validator(|value| {
            if value.parse::<u8>().unwrap() < 2 { return Err("The value must be at least 2.".to_string()); }
            return Ok(());
        })
        .conflicts_with(start_at)
        .conflicts_with(stop_at)
        .conflicts_with(combinations)
        .short("q")
        .long(seq_max)
        .takes_value(true)
        .value_name("num")
        .help("Maximum number of multiple sequential characters");
    let arg_occurrence_max = Arg::with_name(occurrence_max)
        .validator(|value| {
            if value.parse::<u8>().unwrap() < 2 { return Err("The value must be at least 2.".to_string()); }
            return Ok(());
        })
        .conflicts_with(start_at)
        .conflicts_with(stop_at)
        .conflicts_with(combinations)
        .short("r")
        .long(occurrence_max)
        .takes_value(true)
        .value_names(&["num"])
        .help("Maximum number of occurrence of a character");
    let arg_start_at = Arg::with_name(start_at)
        .short("s")
        .long(start_at)
        .takes_value(true)
        .value_name("word")
        .help("Start at specific position");
    let arg_stop_at = Arg::with_name(stop_at)
        .short("l")
        .long(stop_at)
        .takes_value(true)
        .value_name("word")
        .help("Stop at specific position");
    */

    // Arguments help
    let after_help = "* Built-in charsets:
  ?l = abcdefghijklmnopqrstuvwxyz
  ?u = ABCDEFGHIJKLMNOPQRSTUVWXYZ
  ?d = 0123456789
  ?s =  !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~
  ?a = ?l?u?d?s
  ?b = 0x00 - 0xff

* Custom charsets:
  -1,  --custom-charset1=CS  User-definable charsets
  -2,  --custom-charset2=CS  Example:
  -3,  --custom-charset3=CS  --custom-charset1=?dabcdef
  -4,  --custom-charset4=CS  sets charset ?1 to 0123456789abcdef";

    // Arguments
    let arg_compinations = Arg::with_name(combinations)
        .short("c")
        .long(combinations)
        .help("Calculate the number of combinations and exit");
    let arg_file = Arg::with_name(file)
        .short("o")
        .long("output-file")
        .takes_value(true)
        .value_name(file)
        .help("Save the output to the provided file");

    let matches = App::new("\nmaskprocessor-rs (mp)")
        .version("0.1")
        .author("Vagelis Prokopiou <vagelis.prokopiou@gmail.com>")
        .about("High performance word generator with a per-position configurable charset.")
        .after_help(after_help)
        // .arg(arg_increment)      // Todo: Implement this
        // .arg(arg_hex_charset)    // Todo: Implement this
        // .arg(arg_seq_max)        // Todo: Implement this
        // .arg(arg_occurrence_max) // Todo: Implement this
        // .arg(arg_start_at)       // Todo: Implement this
        // .arg(arg_stop_at)        // Todo: Implement this
        .arg(arg_file)
        .arg(arg_compinations)
        .arg(Arg::with_name("custom-charset1")
            .short("1")
            .long("custom-charset1")
            .takes_value(true)
            .value_name(custom_charset_value_name))
        .arg(Arg::with_name("custom-charset2")
            .short("2")
            .long("custom-charset2")
            .takes_value(true)
            .value_name(custom_charset_value_name))
        .arg(Arg::with_name("custom-charset3")
            .short("3")
            .long("custom-charset3")
            .takes_value(true)
            .value_name(custom_charset_value_name))
        .arg(Arg::with_name("custom-charset4")
            .short("4")
            .long("custom-charset4")
            .takes_value(true)
            .value_name(custom_charset_value_name))
        .arg(Arg::with_name(mask)
            .required(true)
            .help("The mask (e.g.: ?a?l?u)"))
        .get_matches();

    // Custom validation.
    /*
    if matches.is_present(increment) {
        validate_increment(&increment);
    }
    */

    /* buffers */
    let mut output_charsets_array: Vec<Vec<char>> = vec![];

    //let mut mp_user = [char; 4];

    // Start processing the mask.
    let mut line_pos = 0;
    let mask_array: Vec<char> = matches.value_of(mask).unwrap().chars().collect();

    while line_pos < mask_array.len() {
        let p0 = mask_array[line_pos];

        if p0 == '?' {
            line_pos += 1;
            let p1 = mask_array[line_pos];

            if p1 == '?' {
                output_charsets_array.push(vec!['?']);
                line_pos += 1;
                continue;
            }
            if p1 == 'l' {
                output_charsets_array.push(Vec::from(CHARSET_LOWER));
                line_pos += 1;
                continue;
            }
            if p1 == 'u' {
                output_charsets_array.push(Vec::from(CHARSET_UPPER));
                line_pos += 1;
                continue;
            }
            if p1 == 'd' {
                output_charsets_array.push(Vec::from(CHARSET_DIGITS));
                line_pos += 1;
                continue;
            }
            if p1 == 's' {
                output_charsets_array.push(Vec::from(CHARSET_SPECIAL));
                line_pos += 1;
                continue;
            }
            if p1 == 'a' {
                output_charsets_array.push(Vec::from(CHARSET_ALL));
                line_pos += 1;
                continue;
            }
            if p1 == 'b' {
                output_charsets_array.push(get_charset_ascii_all());
                line_pos += 1;
                continue;
            }
            // If we reached this point, the mask is wrong.
            eprintln!("Incorrect mask format: ?{}", mask_array[line_pos]);
            exit(1);
        } else {
            let character = mask_array[line_pos as usize];
            output_charsets_array.push(vec![character]);
            line_pos += 1;
        }
    }

    // Combinations
    if matches.is_present(combinations) {
        println!("{}", get_number_of_combinations(&output_charsets_array));
        exit(0);
    }

    // Write to file.
    if matches.is_present(file) {
        let file_value = matches.value_of(file).unwrap();
        let mut file = File::create(file_value)
            .expect(&*format!("Failed to create the {} file", file_value));
        let mut buffer_to_write: Vec<u8> = Vec::with_capacity(get_number_of_combinations(&output_charsets_array) as usize);
        let mut permutations: Vec<char> = vec![];
        recurse_write(&output_charsets_array, 0, &mut permutations, &mut buffer_to_write);
        file.write_all(&*buffer_to_write).expect(&*format!("Failed to write to {}", file_value));
        assert!(buffer_to_write.capacity() >= buffer_to_write.len());
        exit(0);
    }

    // Start execution.
    let mut permutations: Vec<char> = vec![];
    recurse_print(&output_charsets_array, 0, &mut permutations);
}



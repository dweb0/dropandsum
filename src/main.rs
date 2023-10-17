use clap::{App, Arg, ArgMatches};
use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

mod config;
use config::Delimiter;

fn main() {
    let matches = App::new("dropandsum")
        .version(env!("CARGO_PKG_VERSION"))
        .author("dweb0")
        .about("Drop duplicates and sum by column.")
        .args(&[
            Arg::with_name("file")
                .index(1)
                .takes_value(true)
                .help("File to collapse. Use this or pipe input."),
            Arg::with_name("index")
                .long("index")
                .short("i")
                .required(true)
                .takes_value(true)
                .help("The index of the column to collapse on. (Starting from 1, similar to awk)."),
            Arg::with_name("delimiter")
                .long("delimiter")
                .short("d")
                .takes_value(true)
                .default_value("\t")
                .help("The field delimiter that separates the columns. Must be single character."),
            Arg::with_name("sorted")
                .long("sorted")
                .short("s")
                .takes_value(false)
                .help("Sort records by the collapsable column. Default: Unsorted."),
            Arg::with_name("sum-first")
                .long("sum-first")
                .short("f")
                .takes_value(false)
                .help("Output the sum as the first column."),
            Arg::with_name("has-headers")
                .long("has-headers")
                .short("H")
                .takes_value(false)
                .required(false)
                .help("Specify to keep skip header and print in output")
        ])
        .get_matches();

    if let Err(e) = real_main(&matches) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn real_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let delimiter = matches
        .value_of("delimiter")
        .unwrap()
        .parse::<Delimiter>()?;
    let index = matches.value_of("index").unwrap().parse::<usize>()?;
    let sorted = matches.is_present("sorted");
    let sum_first = matches.is_present("sum-first");

    let input: Box<dyn Read> = match matches.value_of("file") {
        Some(file) => Box::new(File::open(file)?),
        None => Box::new(std::io::stdin()),
    };

    let has_headers = matches.is_present("has-headers");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .delimiter(delimiter.as_byte())
        .from_reader(input);

    let headers = if has_headers {
        let headers = rdr
            .headers()
            .unwrap()
            .into_iter()
            .enumerate()
            .filter_map(|(i, rec)| if i != (index - 1) { Some(rec) } else { None })
            .collect::<Vec<_>>()
            .join(&delimiter.as_string());
        Some(headers)
    } else {
        None
    };
    
    let mut collapsed: HashMap<String, usize> = HashMap::new();
    for (line_no, record) in rdr.into_records().enumerate() {
        let record = record?;
        let val = match record.get(index - 1) {
            Some(val) => val.parse::<usize>()?,
            None => {
                return Err(format!(
                    "Could not get value of column {} on line {}",
                    index,
                    line_no + 1
                )
                .into())
            }
        };
        // Skip the nth column and recollect as string
        let rest_of_record = record
            .into_iter()
            .enumerate()
            .filter_map(|(i, rec)| if i != (index - 1) { Some(rec) } else { None })
            .collect::<Vec<_>>()
            .join(&delimiter.as_string());

        *collapsed.entry(rest_of_record).or_insert(0) += val;
    }

    let collapsed: Box<dyn Iterator<Item = (String, usize)>> = if sorted {
        let mut collapsed: Vec<_> = collapsed.into_iter().collect();
        collapsed.sort_by_key(|&(_, count)| Reverse(count));
        Box::new(collapsed.into_iter())
    } else {
        Box::new(collapsed.into_iter())
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    if let Some(headers) = headers {
        writeln!(&mut stdout, "{}", headers).unwrap();
    }

    if sum_first {
        for (rest, count) in collapsed {
            writeln!(&mut stdout, "{}{}{}", count, delimiter, rest).unwrap_or_else(|_| {
                std::process::exit(0);
            })
        }
    } else {
        // Insert the summed value back into its original position
        for (rest, count) in collapsed {
            let mut parts: Vec<_> = rest
                .split(&delimiter.as_string())
                .map(|x| x.to_owned())
                .collect();
            parts.insert(index - 1, count.to_string());
            let new_line = parts.join(&delimiter.as_string());
            writeln!(&mut stdout, "{}", new_line).unwrap_or_else(|_| {
                std::process::exit(0);
            })
        }
    }
    Ok(())
}

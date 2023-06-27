use std::env;
use std::path::Path;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::HashMap;

pub struct Config {
    // file path of Clippings.txt
    pub file_path: String,
    // path to booknotes directory
    pub booknotes_dir: String,
    // flags
    pub fullsync: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();
        let booknotes_dir = args[2].clone();
        let fullsync = true;
        Ok(Config {
            file_path,
            booknotes_dir,
            fullsync,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    let mut input: Vec<String> = Vec::new();
    let mut entries_map: HashMap<String, Vec<Entry>> = HashMap::new();

    for line in contents.lines() {
        // junk the blank lines or delimiters
        if (line.contains("===")) || (line.len() == 0) {
            continue;
        }
        // remove non ascii chars (for some reason kindle inputs a \u{feff} in front of book info)
        input.push(line.replace(|c: char| !c.is_ascii(), ""));
        if input.len() == 3 {
            let res: Entry = parse_delimited_lines(&input);
            let book_vec = entries_map.entry(res.title.clone())
                .or_insert(Vec::new());
            (*book_vec).push(res);

            // println!("{:?}", res);
            // insert_entry(&config.booknotes_dir, &res);
            input.clear();
        }
    }

    for (key, value) in entries_map {
        insert_entries(&config.booknotes_dir, &key, &value);
    }
    // print_entries_map(&entries_map);
    Ok(())
}

fn print_entries_map(entries_map: &HashMap<String, Vec<Entry>>) {
    for (key, value) in entries_map {
        println!("***************");
        println!("Quotations for book: { }", key);
        for entry in value {
            println!("{ }\n", entry.quote);
        }
    }
}

#[derive(PartialEq, Debug)]
struct Entry {
    title: String,
    author: String,
    location: String,
    quote: String,
}

/*
 * Expects: Reference to a vector of Strings containing lines to be parsed.
 * This vector should contain 3 lines.
 * Returns: Returns an Entry object containing the parsed information.
 * TODO: fix error handling, don't return entry, return result
 */
fn parse_delimited_lines(lines: &[String]) -> Entry {
    assert_eq!(lines.len(), 3);

    /* line 1 processing */

    // line 1 contains the title and author
    let mut v: Vec<&str> = lines[0].split('(').collect();
    let title = v[0].strip_suffix(' ').unwrap();
    let author = v[1].strip_suffix(')').unwrap();

    /* line 2 processing */

    // tokenize the string into words
    v = lines[1].split(' ').collect();
    // find the entry after "Location" entry
    let index_of_location_keyword = v.iter().position(|&word| word == "Location").unwrap();
    let location = v[index_of_location_keyword + 1];

    /* line 3 processing */
    let quote = lines[2].clone();

    Entry {
        title: title.to_string(),
        author: author.to_string(),
        location: location.to_string(),
        quote: quote,
    }
}

/*
 * Insert the passed entry into the obsidian notes directory.
 */
fn insert_entries(booknotes_dir: &str, title: &str, entries: &[Entry]) {
    assert!(entries.len() > 0);
    let full_file_path = format!("{}/{}", booknotes_dir, title);
    println!("{full_file_path}");
    let exists = Path::new(&full_file_path).try_exists().expect("Can't check existence");
    let mut file = OpenOptions::new()
        .append(true)
        .create(!exists)
        .open(full_file_path)
        .unwrap();

    for entry in entries {
        let to_write: String = format!("> {}\n", entry.quote);
        writeln!(file, "{}", to_write);
    }
}

// TESTING //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_delimited_lines() {
        let mut input: Vec<String> = Vec::new();
        input.push(String::from("Pilgrim at Tinker Creek (Annie Dillard)"));
        input.push(String::from(
            "- Your Highlight on Location 53-53 |\
                   Added on Tuesday, June 20, 2023 7:58:48 PM",
        ));
        input.push(String::from("finally"));

        let expected = Entry {
            title: String::from("Pilgrim at Tinker Creek"),
            author: String::from("Annie Dillard"),
            location: String::from("53-53"),
            quote: String::from("finally"),
        };
        assert_eq!(expected, parse_delimited_lines(&input));
    }

    #[test]
    fn create_new_file_and_extract() {
        assert!(true);
    }

    #[test]
    fn update_existing_file() {
        assert!(true);
    }
}

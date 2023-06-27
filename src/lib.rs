use std::env;
use std::error::Error;
use std::fs;

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
    /*
     * The clippings format from the kindle is
     *     Book Title (Author)
     *     - line denoting location and date
     *     empty line
     *     Quote all on one line
     *     ========
     *     <feff>Book Title (Author)
     *     - line denoting location and date
     *     empty line
     *     .
     *     .
     *     .
     *     ========
     *     <feff>Book Title (Author)
     *     - line denoting location and date
     *     empty line
     *
     *     Quote all on one line
     *     ======== EOF
     *
     * (1) Create Entry {Title, Author, Quote} by reading 4 lines and discarding
     * junk. Pass the Entry object to a function.
     * (2) This function searches obsidian book-notes folder for a file of the name
     * Book title by author. If no such file is found, create a new file and add the
     * entry to it.
     * */

    Ok(())
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

fn insert_entry(config: &Config) {}

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

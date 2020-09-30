use clap::{App, Arg};
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;

fn main() {
    println!("Hello, user!");

    let filename = "C:/Documenti/markdown-sample.md";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let option = ComrakOptions::default();
    let rendered = markdown_to_html(&contents, &option);
    println!("{}", rendered);
}

/*
struct HelloArgs {
    name: String,
}

impl HelloArgs {

    fn new() -> Self {
        // basic app information
        let app = App::new("hello-clap")
            .version("1.0")
            .author("Me")
            .about("Does awesome things");
        // Extract the actual name

        // Define the name command line option
        let name_option = Arg::with_name("name")
            .long("name") // allow --name
            .short("n") // allow -n
            .takes_value(true)
            .help("Who to say hello to")
            .required(true);

        // now add in the argument we want to parse
        let app = app.arg(name_option);
        // extract the matches
        let matches = app.get_matches_from_safe(args)?;

        // Extract the actual name
        let name = matches
            .value_of("name")
            .expect("This can't be None, we said it was required");

        Ok(HelloArgs {
            name: name.to_string(),
        })
    }
}
*/

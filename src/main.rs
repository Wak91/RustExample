use clap::Clap;
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;

fn main() {
    let opts: HelloOpts = HelloOpts::parse(); // the parse() method is being injected via #[derive(Clap)]

    let filename = opts.input_fname;
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let option = ComrakOptions::default();
    let rendered = markdown_to_html(&contents, &option);
    println!("{}", rendered);
}

// I think the problem you might've had, was that Clap has two
// potential versions to use now: 3.0.0-beta.2 and 2.33.3. In
// version 3 they've considerably changed the way CLI arguments
// are set up. So you might've been following the wrong doc.
// Check out `Cargo.toml` file to see what dependencies you have.
//
// Here's an example of how to do it wiht 3.0.0-*
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Mario Hlaca")]
struct HelloOpts {
    input_fname: String,
}

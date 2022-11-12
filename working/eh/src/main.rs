use clap::{App, Arg};

fn main() {
    let matches = App::new("eh")
        .version("0.1.0")
        .author("Alex Vyber <alexvyber@mail.com>")
        .about("Rust echo. Blazingly fast!")
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    // let other_text = vec!["some", "text"];

    // println!("{:#?} and omit is: {:#?}", text, omit_newline);
    // println!("{}", other_text.join(" "));
    // println!("{}", text.join(" "));

    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}

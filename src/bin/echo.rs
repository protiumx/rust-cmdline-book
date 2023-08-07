fn main() {
    let m = clap::Command::new("echo")
        .author("protium, ioprotium@gmail.com")
        .version("0.1.0")
        .about("Echoes arguments")
        .arg(clap::Arg::new("txt").num_args(0..))
        .arg(
            clap::Arg::new("omit_newline")
                .short('n')
                .num_args(0)
                .action(clap::ArgAction::SetTrue)
                .required(false),
        )
        .get_matches();
    let ending = if m.get_flag("omit_newline") { "" } else { "\n" };
    let txt = if let Some(args) = m.get_many::<String>("txt") {
        args.cloned().collect::<Vec<_>>().join(" ")
    } else {
        "".to_string()
    };

    print!("{}{}", txt, ending);
}

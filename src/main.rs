pub use clap::{command, Arg};
mod display;

fn arg_parse() -> clap::Command 
{
    command!()
        .about("Intuitive homelab monitoring tool")
        .version("0.1")
        .arg(Arg::new("interface")
            .short('i')
            .long("interface")
            .required(true)
        )
        .arg(Arg::new("group")
            .short('g')
            .long("group")
            .required(true)
        )
        .arg(Arg::new("lab_name")
            .short('n')
            .long("lab_name")
            .required(false)
            .default_value("Homelab")
        )
}

fn main() {
    let matches = arg_parse().get_matches();
    println!("interface is : {:?}", matches.get_one::<String>("interface"));

    let lab_name = matches.get_one::<String>("lab_name");
    dbg!(lab_name);

    let _result = display::draw("Homelab".to_string());
}

#[test]
fn verify_cli() {
    arg_parse().debug_assert();
}

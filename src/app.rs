use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("json")
                .help("Print weather as json")
                .long_help(
                    "Takes an argument of 'pretty', which will pretty print the json output.",
                )
                .long("json")
                .min_values(0)
                .takes_value(true)
                .value_name("pretty"),
        )
}

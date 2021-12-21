use clap::{App, Arg};
use discord_data_markov_chain::entrypoint::entrypoint;
use std::path::Path;

fn main() {
    let args = App::new("Discord Data Markov Chains")
        .version(env!("CARGO_PKG_VERSION"))
        .author("0/0#0001")
        .arg(
            Arg::with_name("msg_dir")
                .short("m")
                .long("msg-dir")
                .value_name("MSG_DIR")
                .help("Path to your `messages` directory in your Discord data package.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("COUNT")
                .help("How many chain messages to show.")
                .takes_value(true)
                .default_value("100")
                .validator(|x| {
                    x.parse::<u32>()
                        .map(|_| ())
                        .map_err(|_| "couldn't parse count as a integer".to_string())
                }),
        )
        .arg(
            Arg::with_name("skip")
                .long("skip")
                .multiple(true)
                .value_name("SKIP")
                .help("skip adding messages with this content")
                .takes_value(true)
        )
        .get_matches();

    let to_skip = args.values_of("count").expect("something went wrong").collect();
    entrypoint(
        &Path::new(args.value_of("msg_dir").expect("something went wrong")),
        args.value_of("count")
            .expect("something went wrong")
            .parse()
            .expect("something went wrong"),
        to_skip
    );
}

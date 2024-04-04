use callframe::Callframe;
use clap::{command, Arg, parser::ValueSource};
use futures::executor::block_on;

mod callframe;

#[tokio::main]
async fn main() {

    let match_result = command!()
    .about("quick-api: A command line interface for prototyping API calls")
    .arg(Arg::new("new")
        .short('n')
        .long("new")
        .num_args(0)
        .exclusive(true)
        .help("Construct a new API call")
    ).arg(
        Arg::new("load")
        .short('l')
        .long("load")
        .num_args(0)
        .exclusive(true)
        .help("Select from existing saved API calls")
    ).get_matches();

    if match_result.value_source("new") == Some(ValueSource::CommandLine) {
        let mut callframe = callframe::new::new_callframe();

        let future = callframe.make_request();
        let _ = block_on(future);

        let _ = Callframe::save_callframe(&callframe);
    }

    else if match_result.value_source("load") == Some(ValueSource::CommandLine) {
        let loaded_result: Option<Callframe> = callframe::load::load_callframe();

        match loaded_result {
            Some(mut callframe) => {
                callframe.name = "test".to_string();
                let future = callframe.make_request();
                let _ = block_on(future);
                let _ = Callframe::save_callframe(&callframe);
            }
            None => {println!("Failed to load in callframe");}
        }
    }
}

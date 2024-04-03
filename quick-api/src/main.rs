use callframe::Callframe;
use clap::{command, Arg};
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

    if match_result.contains_id("new") {
        let mut callframe = callframe::new::new_callframe();

        let future = callframe.make_request();
        let _ = block_on(future);

        let _ = Callframe::save_callframe(&callframe);
    }

    if match_result.contains_id("load") {
        println!("test");
        let loaded_result: Option<Callframe> = callframe::load::load_callframe();

        match loaded_result {
            Some(mut callframe) => {
                let future = callframe.make_request();
                let _ = block_on(future);
                let _ = Callframe::save_callframe(&callframe);
            }
            None => {println!("Failed to load in callframe");}
        }
    }
}

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
        .help("Construct a new API call")
    ).get_matches();

    if match_result.contains_id("new") {
        let mut callframe = callframe::new::new_callframe();

        let future = callframe.make_request();
        let _ = block_on(future);

        let _ = Callframe::save_callframe(&callframe);
    }

}

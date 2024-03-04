use futures::executor::block_on;

mod api_handler;
mod callframe;

#[tokio::main]
async fn main() {
    println!("quick-api: A command line interface for prototyping API calls");

    loop {
        let mut callframe: callframe::Callframe = callframe::Callframe::build_callframe();

        let future = callframe.make_request();
        let _ = block_on(future);
    }
}

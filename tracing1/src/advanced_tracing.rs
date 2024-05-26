use tracing::{info, Instrument};
use tracing_futures::WithSubscriber;
// use tracing_serde::AsSerde;
// use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_env_filter("my_crate=info")
        .finish();

    let _guard = tracing::subscriber::set_default(subscriber);

    let future = async {
        info!("This is an async block");
        println!("hello, async world!");
    };

    let subscriber2 = tracing_subscriber::fmt()
        .json()
        .with_env_filter("my_crate=info")
        .finish();
    future
        .instrument(tracing::info_span!("my_span"))
        .with_subscriber(subscriber2)
        .await;

    // let span = tracing::info_span!("my_span");
    // println!("{}", serde_json::to_string(&span.as_serde()).unwrap());

    println!("done");
}

use tracing::{error, info, trace, warn};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    /* tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new())
    .expect("setting default subscriber failed"); */

    let number = 5;
    info!("The number is {}", number);

    let result = compute(number);
    info!("The result is {}", result);

    println!("done");
}

fn compute(n: i32) -> i32 {
    trace!("Computing the value...");

    if n > 10 {
        warn!("The number is greater than 10");
    } else if n < 1 {
        error!("The number is less than 1");
    }

    n * 2
}

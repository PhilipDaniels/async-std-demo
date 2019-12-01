use async_std::task;
use futures::join;
use futures::stream::{FuturesUnordered, StreamExt};
use std::time::{Duration, Instant};
use std::thread;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let start_time = Instant::now();

    demo_waiting_for_two_async_fns();

    println!("Program finished in {} ms", start_time.elapsed().as_millis());
}

fn demo_waiting_for_two_async_fns() {
    // block_on takes a future and waits for it to complete.
    // Notice that this fn is not `async`, and we are not using
    // an async block either (because we are not calling `await`).
    task::block_on(call_both_sleepers());
}

async fn call_both_sleepers() {
    join!(first_sleeper(), second_sleeper());
}

async fn first_sleeper() {
    sleep_and_print(1, 1000).await;
}

async fn second_sleeper() {
    sleep_and_print(2, 1500).await;
}

/// This utility function simply goes to sleep for a specified time
/// and then prints a message when it is done.
async fn sleep_and_print(future_number: u32, sleep_millis: u64) {
    let sleep_duration = Duration::from_millis(sleep_millis);
    // Note we are using async-std's `task::sleep` here, not
    // thread::sleep. We must not block the thread!
    task::sleep(sleep_duration).await;
    println!("Future {} slept for {} ms on {:?}", future_number, sleep_millis, thread::current().id());
}

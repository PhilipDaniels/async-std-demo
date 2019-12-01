use async_std::task;
use futures::join;
use futures::stream::{FuturesUnordered, StreamExt};
use std::time::{Duration, Instant};
use std::thread;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let start_time = Instant::now();

    //demo_waiting_for_two_async_fns();
    demo_waiting_for_multiple_random_sleeps();

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

fn demo_waiting_for_multiple_random_sleeps() {
    // Initialise the random number generator we will use to
    // generate the random sleep times.
    let between = Uniform::from(500..10_000);
    let mut rng = rand::thread_rng();

    // This special collection from the `futures` crate is what we use to
    // hold all the futures; it is designed to efficiently poll the futures
    // until they all complete, (in any order) which we do with a simple
    // loop (see below).
    let mut futures = FuturesUnordered::new();

    // Create 10 futures, each of which should sleep for a random
    // number of milliseconds. None of the futures are doing anything
    // yet, because we are only storing them; we haven't started polling
    // them yet.
    for future_number in 0..10 {
        let sleep_millis = between.sample(&mut rng);
        futures.push(sleep_and_print(future_number, sleep_millis));
    }

    // This loop is how to wait for all the elements in a `FuturesUnordered<T>`
    // to complete. `value_returned_from_the_future` is just the
    // unit tuple, `()`, because we did not return anything from `sleep_and_print`.
    task::block_on(async {
        while let Some(_value_returned_from_the_future) = futures.next().await {
        }
    });
}

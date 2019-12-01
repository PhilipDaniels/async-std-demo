use async_std::task;
use futures::join;
use futures::stream::{FuturesUnordered, StreamExt};
use std::time::{Duration, Instant};
use std::thread;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let start_time = Instant::now();

    //demo_waiting_for_two_async_fns();
    //demo_waiting_for_multiple_random_sleeps();
    //demo_waiting_for_multiple_random_sleeps_with_return_values();
    //demo_waiting_for_multiple_random_sleeps_with_errors();
    demo_downloading_urls();

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

async fn sleep_and_print_and_return_value(future_number: u32, sleep_millis: u64) -> u32 {
    let sleep_duration = Duration::from_millis(sleep_millis);
    task::sleep(sleep_duration).await;
    println!("Future {} slept for {} ms on thread {:?}", future_number, sleep_millis, thread::current().id());

    future_number * 10
}

fn demo_waiting_for_multiple_random_sleeps_with_return_values() {
    let between = Uniform::from(500..10_000);
    let mut rng = rand::thread_rng();

    let mut futures = FuturesUnordered::new();

    for future_number in 0..10 {
        let random_millis = between.sample(&mut rng);
        futures.push(sleep_and_print_and_return_value(future_number, random_millis));
    }

    // The async block borrows a mutable reference to `sum`, allowing us to
    // add up all the values returned from the future.
    let mut sum = 0;
    task::block_on(async {
        while let Some(value_returned_from_the_future) = futures.next().await {
            sum += value_returned_from_the_future;
        }
    });

    println!("Sum of all values returned = {}", sum);
}

async fn sleep_and_print_and_return_error(future_number: u32, sleep_millis: u64) -> Result<u32, String> {
    let sleep_duration = Duration::from_millis(sleep_millis);
    task::sleep(sleep_duration).await;
    println!("Future {} slept for {} ms on thread {:?}", future_number, sleep_millis, thread::current().id());

    if future_number % 2 == 0 {
        Ok(future_number * 10)
    } else {
        Err(format!("It didn't work for future {}", future_number))
    }
}

fn demo_waiting_for_multiple_random_sleeps_with_errors() {
    let between = Uniform::from(500..10_000);
    let mut rng = rand::thread_rng();

    let mut futures = FuturesUnordered::new();

    for future_number in 0..10 {
        let random_millis = between.sample(&mut rng);
        futures.push(sleep_and_print_and_return_error(future_number, random_millis));
    }

    // Now, `value_returned_from_the_future` is a `Result<u32, String>` so
    // we must take care to pattern match on it.
    let mut sum = 0;
    task::block_on(async {
        while let Some(value_returned_from_the_future) = futures.next().await {
            match value_returned_from_the_future {
                Ok(value) => sum += value,
                Err(e) => println!("    Got error back: {}", e),
            }
        }
    });

    println!("Sum of all values returned = {}", sum);
}

async fn download_url(url: &str) -> Result<String, surf::Exception> {
    println!("Downloading {} on thread {:?}", url, thread::current().id());

    // Code taken directly from the example for `surf`.
    let mut result = surf::get(url).await?;
    let body = result.body_string().await?;

    println!("    Downloaded {}, returning body of length {} ", url, body.len());
    Ok(body)
}

fn demo_downloading_urls() {
    let urls = vec![
        "https://www.sharecast.com/equity/Anglo_American",
        "https://www.sharecast.com/equity/Associated_British_Foods",
        "https://www.sharecast.com/equity/Admiral_Group",
        "https://www.sharecast.com/equity/Aberdeen_Asset_Management",
        "https://www.sharecast.com/equity/Aggreko",
        "https://www.sharecast.com/equity/Ashtead_Group",
        "https://www.sharecast.com/equity/Antofagasta",
        "https://www.sharecast.com/equity/Aviva",
        "https://www.sharecast.com/equity/AstraZeneca",
        "https://www.sharecast.com/equity/BAE_Systems",
        "https://www.sharecast.com/equity/Babcock_International_Group",
        "https://www.sharecast.com/equity/British_American_Tobacco",
        "https://www.sharecast.com/equity/Balfour_Beatty",
        "https://www.sharecast.com/equity/Barratt_Developments",
        "https://www.sharecast.com/equity/BG_Group",
        "https://www.sharecast.com/equity/British_Land_Company",
        "https://www.sharecast.com/equity/BHP_Group",
        "https://www.sharecast.com/equity/Bunzl",
        "https://www.sharecast.com/equity/BP",
        "https://www.sharecast.com/equity/Burberry_Group",
        "https://www.sharecast.com/equity/BT_Group",
    ];

    // This time let's make our FuturesUnordered value by collecting
    // a set of futures.
    let mut futures = urls.iter()
        .map(|url| download_url(url))
        .collect::<FuturesUnordered<_>>();

    task::block_on(async {
        while let Some(return_val) = futures.next().await {
            match return_val {
                Ok(body) => {
                    // Possibly do something useful with the body of the request here.
                },
                Err(e) => println!("    Got error {:?}", e),
            }
        }
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = sentry::init((
        "https://56b5cf6ec6d04220d1ba9dfbacb720dd@o1413557.ingest.us.sentry.io/4507903000576000",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // implementation of main
            println!("Hello, world!");

            if let Err(err) = std::panic::catch_unwind(|| {
                panic!("Everything is on fire!");
            }) {
                eprintln!("Oops! Something went wrong: {:?}", err);
                // Additional error handling code goes here
            }
        });

    Ok(())

}

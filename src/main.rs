use arbitrary::Arbitrary;
use arbitrary::Unstructured;
use clap::Parser;

use database::cleanup::cleanup_database;
use models::cli::Cli;
use musicbrainz_db_lite::entity::recording::Recording;
use tracing::debug;

pub mod api;
pub mod database;
pub mod datastructures;
pub mod interface;
pub mod models;
#[cfg(test)]
pub mod testing;
/// This is the module containing all the different tools of this app
pub mod tools;
pub mod utils;

use crate::api::clients::create_client;
use crate::api::clients::ALISTRAL_CLIENT;
use crate::interface::tracing::init_tracer;
pub use crate::models::error::Error;

#[derive(Arbitrary, Debug)]
struct TestData {
    listens: Vec<UserListensResponse>,
    recordings: Vec<Recording>
}

#[tokio::main]
async fn main() {
    let bytes = "jsfpijeosqk$qk$$oesk$poejkspjrijriojgjdriojdroij";
    
    let mut unstruct = Unstructured::new(bytes);

    println!("{:#?}", TestData::arbitrary_take_rest(unstruct));

    panic!();




    let cli = Cli::parse();
    init_tracer(&cli);

    if run_cli(cli).await {
        post_run().await
    }
}

async fn run_cli(cli: Cli) -> bool {
    debug!("Starting the database");
    // Set up the database
    let conn = &mut *ALISTRAL_CLIENT
        .musicbrainz_db
        .connection
        .acquire_guarded()
        .await;

    match cli.run(conn).await {
        Result::Ok(val) => val,
        Err(err) => err.panic(),
    }
}

async fn post_run() {
    let alistral_client = create_client().await;
    let conn = &mut *alistral_client
        .musicbrainz_db
        .connection
        .acquire_guarded()
        .await;

    debug!("Cleaning some old entries...");
    cleanup_database(conn)
        .await
        .expect("Error while cleaning the database");
    debug!("Done!");
}

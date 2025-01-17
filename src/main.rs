use alistral_core::cli::logger::println_cli;
use clap::Parser;
use color_eyre::eyre::Ok;

use database::cleanup::cleanup_database;
use database::get_conn;
use models::cli::Cli;

pub mod api;
pub mod database;
pub mod datastructures;
pub mod models;
#[cfg(test)]
pub mod testing;
/// This is the module containing all the different tools of this app
pub mod tools;
pub mod utils;

pub use crate::models::error::Error;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Set up the database
    let mut conn = get_conn().await;

    let cli = Cli::parse();

    let post_run = cli
        .run(&mut conn)
        .await
        .expect("An error occured in the app");

    // The details of the connection may have changed. We recreate the connection
    let mut conn = get_conn().await;

    if post_run {
        println_cli("Optional cleanup - This is fine to cancel");
        println_cli("Cleaning some old entries...");
        cleanup_database(&mut conn)
            .await
            .expect("Error while cleaning the database");
        println_cli("Done!");
    }

    println_cli("Have a nice day!");
    Ok(())
}

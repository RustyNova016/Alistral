use clap::Parser;

use crate::interface::tracing::init_tracer;
use models::cli::Cli;

pub(crate) mod api;
pub(crate) mod database;
pub(crate) mod datastructures;
pub(crate) mod interface;
pub(crate) mod models;
#[cfg(test)]
pub(crate) mod testing;
/// This is the module containing all the different tools of this app
pub(crate) mod tools;
pub(crate) mod utils;

pub use crate::models::client::ALISTRAL_CLIENT;
pub use crate::models::error::Error;

//pub(crate) type ColEyreVal<T> = color_eyre::Result<T>;
pub(crate) type ColEyre = color_eyre::Result<()>;

#[tokio::main]
async fn main() -> ColEyre {
    let _ = dotenvy::dotenv();
    color_eyre::install()?;
    let cli = Cli::parse();
    let _worker_guard = init_tracer(&cli);
    //console_subscriber::init();

    match cli.run().await {
        Result::Ok(clean) => {
            if clean {
                ALISTRAL_CLIENT.clean_up_mb_db().await;
            }
        }
        Err(err) => err.panic(),
    }

    Ok(())
}

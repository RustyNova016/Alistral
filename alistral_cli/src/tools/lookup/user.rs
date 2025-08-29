use core::fmt::Display;
use std::fmt::Write;

use alistral_core::datastructures::entity_with_listens::listen_timeframe::ListenTimeframe;
use alistral_core::datastructures::entity_with_listens::user::UserWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::models::user::UserData;
use chrono::DateTime;
use chrono::Utc;
use clap::Parser;

use tuillez::OwoColorize as _;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::common::Timeframe;
use crate::models::config::Config;
use crate::tools::lookup::components::LookupLine;

#[derive(Parser, Clone, Debug)]
pub(super) struct LookupUserCommand {
    /// Get the data of this user
    user: Option<String>,

    #[clap(short, long)]
    timeframe: Option<Timeframe>,
}

impl LookupUserCommand {
    pub async fn run(&self) {
        let user = Config::check_username(&self.user);

        let userdata = UserData::load_user(&ALISTRAL_CLIENT.core, user)
            .await
            .expect("Couldn't load user data");

        let end = Utc::now();

        let start = match self.timeframe {
            Some(v) => v.get_start_date(),
            None => DateTime::from_timestamp(0, 0).unwrap(),
        };

        let timeframe = ListenTimeframe::new(start, end, userdata.user_with_listens().clone());

        print_report(&timeframe).await
    }
}

fn get_title(data: &ListenTimeframe<UserWithListens>) -> String {
    if data.previous_opt().is_some() {
        format!(
            "   Statistics for {} {}",
            data.current().entity().name,
            format!(
                "({} -> {}, compared to {} -> {})",
                data.start().format("%d/%m/%Y"),
                data.end().format("%d/%m/%Y"),
                data.prev_start().format("%d/%m/%Y"),
                data.start().format("%d/%m/%Y"),
            )
            .bright_black()
        )
        .on_green()
        .black()
        .bold()
        .to_string()
    } else {
        format!("All time statistics for {}", data.current().entity().name)
            .on_green()
            .black()
            .bold()
            .to_string()
    }
}

async fn get_listencount(data: &ListenTimeframe<UserWithListens>) -> impl Display {
    LookupLine::builder()
        .description("Listen count".to_string())
        .data(data.clone())
        .get_data(|ent| ent.listen_count())
        .build()
        .to_string()
        .await
}

async fn print_report(data: &ListenTimeframe<UserWithListens>) {
    let mut out = String::new();

    writeln!(out, "{}", get_title(data)).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "{}", get_listencount(data).await).unwrap();

    println!("{out}");
}

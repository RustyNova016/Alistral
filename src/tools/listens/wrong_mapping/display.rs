use color_eyre::owo_colors::OwoColorize as _;
use inquire::InquireError;
use inquire::Select;
use listenbrainz::raw::Client;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use strsim::sorensen_dice;

use crate::models::config::whitelisted_wrong_mappings::WhilistedWrongMappings;
use crate::utils::cli::display::RecordingExt as _;
use crate::utils::cli::hyperlink_rename;
use crate::utils::extensions::owo_colors_ext::AlistralColors as _;

pub(super) async fn display_wrong_mapping(
    conn: &mut sqlx::SqliteConnection,
    config: &mut WhilistedWrongMappings,
    messybrainz_data: &MessybrainzSubmission,
    recording: &Recording,
    listen: &Listen,
) -> bool {
    println!();
    println!("{}", "Wrong mapping".to_string().as_title());
    println!();
    println!(
        "Listen data {} by {} is currently mapped to {}",
        messybrainz_data.recording.truecolor(0, 184, 84),
        messybrainz_data.artist_credit.truecolor(0, 143, 229),
        recording
            .pretty_format_with_credits(conn, true)
            .await
            .expect("Couldn't format credits")
    );
    println!();

    let title_score = sorensen_dice(
        &messybrainz_data.recording.to_lowercase(),
        &recording.title.to_lowercase(),
    );
    if title_score == 1.0 {
    } else if title_score < 0.5 {
        println!("Title similarity: {}", title_score.red());
    } else {
        println!("Title similarity: {}", title_score.yellow());
    }

    let artist_score = sorensen_dice(
        &messybrainz_data.artist_credit.to_lowercase(),
        &recording
            .get_artist_credits_or_fetch(conn)
            .await
            .expect("Couldn't get the artist credit")
            .to_string()
            .to_lowercase(),
    );
    if artist_score == 1.0 {
    } else if artist_score < 0.5 {
        println!("Artist similarity: {}", artist_score.red());
    } else {
        println!("Artist similarity: {}", artist_score.yellow());
    }

    println!();
    println!(
        "{}",
        hyperlink_rename(
            &"See listen on listenbrainz".to_string(),
            &format!(
                "https://listenbrainz.org/user/RustyNova/?max_ts={}",
                listen.listened_at + 1
            )
        )
    );
    println!();

    match choice() {
        Choice::Next => {
            let lb_client = Client::new();
            Listen::fetch_listen_by_id(
                conn,
                &lb_client,
                listen.listened_at,
                &listen.user,
                &listen.recording_msid,
                10,
            )
            .await
            .expect("Couldn't refresh listen");
            true
        }
        Choice::Whitelist => {
            config.add(messybrainz_data.msid.clone(), recording.mbid.clone());
            true
        }
        Choice::Exit => false,
    }
}

#[derive(strum_macros::Display)]
enum Choice {
    Next,
    #[strum(to_string = "Whitelist mapping")]
    Whitelist,
    Exit,
}

fn choice() -> Choice {
    loop {
        let options = vec![Choice::Next, Choice::Whitelist, Choice::Exit];

        let ans = Select::new("", options).prompt();

        match ans {
            Ok(choice) => return choice,
            Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
                return Choice::Exit
            }
            _ => println!("There was an error, please try again"),
        }
    }
}

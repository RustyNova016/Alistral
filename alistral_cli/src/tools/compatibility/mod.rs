use user_compatibility::get_shared_ratio;
use user_compatibility::get_shared_recordings_between_users;
use user_compatibility::get_user_shared_percent;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::recording_stats;
use crate::utils::cli::await_next;

pub mod user_compatibility;

pub async fn compatibility_command(user_a: &str, user_b: &str) {
    let user_a_recordings = recording_stats(&ALISTRAL_CLIENT, user_a.to_string())
        .await
        .expect("Couldn't get the listened recordings");
    let user_b_recordings = recording_stats(&ALISTRAL_CLIENT, user_a.to_string())
        .await
        .expect("Couldn't get the listened recordings");

    let shared_recordings =
        get_shared_recordings_between_users(&user_a_recordings, &user_b_recordings);

    println!(
        "
Compatibility results:

[Shared Recordings]
    There is {} recordings both listened by {user_a} and {user_b}
        > This is {}% of {user_a}'s listened recordings
        > This is {}% of {user_b}'s listened recordings

[Compatibility]
    The compatibilty score between {user_a} and {user_b} is {}%
    ",
        shared_recordings.len(),
        get_user_shared_percent(&shared_recordings, &user_a_recordings).trunc_with_scale(2),
        get_user_shared_percent(&shared_recordings, &user_b_recordings).trunc_with_scale(2),
        get_shared_ratio(&shared_recordings, &user_a_recordings, &user_b_recordings)
    );

    await_next();
}

// #[tokio::test]
// #[serial_test::serial]
// async fn compatibility() {
//     compatibility_command("RustyNova", "backhdlp").await;
// }

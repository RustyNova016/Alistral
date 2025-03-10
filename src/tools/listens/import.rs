use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
use musicbrainz_db_lite::models::listenbrainz::msid_mapping::MsidMapping;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::user::User;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Acquire;
use tracing::info;

pub async fn import_listen_dump(
    conn: &mut sqlx::SqliteConnection,
    dump_path: &Path,
    username: &str,
) {
    let zip_file = File::open(dump_path).expect("Couldn't access zip file.");
    let mut archive = zip::ZipArchive::new(zip_file).expect("Couldn't read zip file.");

    let mut import_trans = conn.begin().await.expect("Couldn't start transaction");

    // We read the zip file
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        // The file is a directory? Skip. We don't need to handle those
        if file.is_dir() {
            continue;
        }

        // The file is actually a listen?
        if outpath.to_string_lossy() == "feedback.jsonl"
            || outpath.to_string_lossy() == "pinned_recording.jsonl"
            || outpath.to_string_lossy() == "user.json"
        {
            continue;
        }

        println!("Saving {}", outpath.display());

        // Convert jsonl to json
        let content = BufReader::new(file).lines();

        // Then save the content
        let mut count = 0;
        let mut trans = import_trans
            .begin()
            .await
            .expect("Couldn't start transaction");
        for line in content {
            let line = line.expect("Couldn't read line");
            //println!("{line}");
            let data: ImportListen = serde_json::from_str(&line).unwrap_or_else(|err| {
                panic!(
                    "Couldn't convert line #{} of {}. Error: {err}",
                    count + 1,
                    outpath.display()
                )
            });

            data.save(&mut trans, username)
                .await
                .expect("Couldn't save listen");
            count += 1;
        }
        trans.commit().await.expect("Couldn't save transaction");

        info!("Loaded {count} listens");
    }
    import_trans
        .commit()
        .await
        .expect("Couldn't save transaction");
}

//TODO: #449 Move ImportListen to models
#[derive(Debug, Deserialize, Serialize)]
struct ImportListen {
    listened_at: i64,
    track_metadata: ImportListenMetaData,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImportListenMetaData {
    track_name: String,
    artist_name: String,
    release_name: Option<String>,
    recording_msid: String,
    additional_info: HashMap<String, serde_json::Value>,
    mbid_mapping: Option<ImportListenMapping>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImportListenMapping {
    caa_id: Option<u64>,
    caa_release_mbid: Option<String>,
    artists: Vec<ImportListenMappingArtists>,
    artist_mbids: Vec<String>,
    release_mbid: Option<String>,
    recording_mbid: String,
    recording_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImportListenMappingArtists {
    artist_mbid: String,
    join_phrase: String,
    artist_credit_name: String,
}

impl ImportListen {
    pub async fn save(
        self,
        conn: &mut sqlx::SqliteConnection,
        user_name: &str,
    ) -> Result<(), crate::Error> {
        // First, get the user
        User::insert_or_ignore(&mut *conn, user_name).await.unwrap();

        let data = serde_json::to_string(&self.track_metadata.additional_info)
            .expect("Crashing from serializing a serde::Value isn't possible");

        let messybrainz = MessybrainzSubmission {
            id: 0,
            msid: self.track_metadata.recording_msid.clone(),
            recording: self.track_metadata.track_name,
            artist_credit: self.track_metadata.artist_name,
            release: self.track_metadata.release_name,
            track_number: None, // TODO: Find where is it stored in the json... If it even is stored...
            duration: None, //TODO: Get the duration from additiona info or ditch it from the schema?
        };

        messybrainz.insert_or_ignore(&mut *conn).await.unwrap();

        if let Some(mapping) = self.track_metadata.mbid_mapping {
            // First insert the mbid
            Recording::add_redirect_mbid(conn, &mapping.recording_mbid)
                .await
                .unwrap();

            let user = User::find_by_name(&mut *conn, user_name)
                .await?
                .expect("The user shall be inserted");

            MsidMapping::set_user_mapping(
                &mut *conn,
                user.id,
                self.track_metadata.recording_msid.clone(),
                mapping.recording_mbid.to_string(),
            )
            .await?;
        }

        let listen = Listen {
            id: 0,
            listened_at: self.listened_at,
            user: user_name.to_string(),
            recording_msid: self.track_metadata.recording_msid.clone(),
            data: Some(data),
        };

        listen.upsert_listen(conn).await.unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ALISTRAL_CLIENT;
    use crate::tools::listens::import::import_listen_dump;
    use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
    use std::path::PathBuf;

    #[sqlx::test]
    async fn load_listen_dump_test() {
        let mut conn = ALISTRAL_CLIENT
            .musicbrainz_db
            .get_raw_connection()
            .await
            .expect("Couldn't connect to the database");
        import_listen_dump(
            &mut conn,
            &PathBuf::from("tests/data/listen_dump.zip".to_string()),
            "TestNova",
        )
        .await;

        //TODO: #451 Make sqlx prepare query macros in tests + Convert the queries
        let listen: Listen = sqlx::query_as("SELECT * FROM listens WHERE listened_at = 1705054374")
            .fetch_one(&mut *conn)
            .await
            .expect("This listen should exist");
        listen
            .get_recording_or_fetch(&mut conn, &ALISTRAL_CLIENT.musicbrainz_db)
            .await
            .expect("The listen should be mapped");
    }
}

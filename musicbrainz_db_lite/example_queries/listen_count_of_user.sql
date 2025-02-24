-- database: ../examples/load_recordings_of_listens/db.db
SELECT COUNT(*) as listen_count,
    recordings.id,
    recordings.title
FROM users
    INNER JOIN listens ON users.name = listens.user
    INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
    INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
    INNER JOIN recordings ON recordings_gid_redirect.new_id = recordings.id
WHERE users.id = ?
GROUP BY recordings.id
ORDER BY listen_count DESC;
-- database: ./tests/test_db.db

SELECT Count(listens.id) as listen_count,
    messybrainz_submission.recording,
    messybrainz_submission.artist_credit
FROM listens
    INNER JOIN messybrainz_submission on listens.recording_msid = messybrainz_submission.msid
GROUP BY messybrainz_submission.msid
-- Add migration script here
DROP INDEX `idx_external_id`;

-- Remove the dupes
DELETE FROM external_id WHERE id in (SELECT
    id
FROM
    external_id
WHERE
    external_id.id NOT IN (
        SELECT
            id
        FROM
            (
                SELECT
                    COUNT(recording_id) AS dupes,
                    id
                FROM
                    external_id
                GROUP BY
                    recording_id,
                    service,
                    user_overwrite
                HAVING
                    dupes <= 1
            )
    )
    AND external_id.id NOT IN (
        SELECT
            id
        FROM
            (
                SELECT
                    MIN(id) AS id,
                    COUNT(recording_id) AS dupes
                FROM
                    external_id
                GROUP BY
                    recording_id,
                    service,
                    user_overwrite
                HAVING
                    dupes >= 2
            )
    ));

CREATE UNIQUE INDEX `idx_external_id` ON `external_id` (`recording_id`, `service`, `user_overwrite`)
-- Add migration script here
CREATE TABLE `isrcs` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL, 
    `isrc` TEXT NOT NULL, 
    `recording` INTEGER NOT NULL REFERENCES `recordings`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX `unique_recording_isrc` ON `isrcs` (`isrc`, `recording`)
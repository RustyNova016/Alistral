-- Add migration script here
DELETE FROM `works` WHERE SUBSTRING(work_type, 1, 1) = '"'
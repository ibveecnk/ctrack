-- Your SQL goes here
CREATE TABLE `users`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`username` TEXT NOT NULL,
	`email` TEXT NOT NULL,
	`password` TEXT NOT NULL,
	`unique_id` TEXT NOT NULL
);


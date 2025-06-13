-- Your SQL goes here
CREATE TABLE user_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL
);

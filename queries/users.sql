-- queries/users.sql

--! get_all_users
SELECT id, name FROM users;

--! insert_user
INSERT INTO users (name) VALUES (:name) RETURNING id, name;

-- queries/users.sql

--! get_all_users
SELECT id, name FROM users;

--! insert_user
INSERT INTO users (name) VALUES (:name) RETURNING id, name;

--! get_user
SELECT id, name FROM users WHERE id = :id;

--! delete_user
DELETE FROM users WHERE id = :id RETURNING id, name;

--! update_user
UPDATE users SET name = :name WHERE id = :id RETURNING id, name;

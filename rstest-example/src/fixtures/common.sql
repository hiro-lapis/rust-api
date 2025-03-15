-- create users table
-- CREATE TABLE IF NOT EXISTS users (
--     user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
--     name VARCHAR(255) NOT NULL,
--     email VARCHAR(255) NOT NULL UNIQUE,
--     password_hash VARCHAR(255) NOT NULL,
--     role_id UUID NOT NULL,
--     created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
--     updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),

--   FOREIGN KEY (role_id) REFERENCES roles(role_id)
--   ON UPDATE CASCADE
--   ON DELETE CASCADE
-- );

INSERT INTO roles (name) VALUES ('admin'), ('user');
INSERT INTO users (name, email, password_hash, role_id) VALUES ('hiro-admin', ' example0@example.com', 'password', (SELECT role_id FROM roles WHERE name = 'admin'));
INSERT INTO users (name, email, password_hash, role_id) VALUES ('hiro-user', 'example1@example.com', 'password', (SELECT role_id FROM roles WHERE name = 'user'));

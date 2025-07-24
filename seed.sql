-- Insert admin user
INSERT INTO users (id, username, email, password_hash)
VALUES (
    '4d3f3b89-4fc3-4db8-995d-0d27ceac520e',
    'admin',
    'admin@example.com',
    '$argon2id$v=19$m=19456,t=2,p=1$HKnvmOkOIHTKtdjOX1RIzA$qnMDntwvrgWAP/QExEjgnKKaHw0+3jqkEPLJt8B6NIg'
)
ON CONFLICT ON CONSTRAINT users_username_key DO UPDATE
SET email = EXCLUDED.email,
    password_hash = EXCLUDED.password_hash;

-- Insert admin role
INSERT INTO roles (id, name, description)
VALUES (
    '223e4567-e89b-12d3-a456-426614174001',
    'admin',
    'Administrator role with full privileges'
)
ON CONFLICT ON CONSTRAINT roles_name_key DO UPDATE
SET description = EXCLUDED.description;

-- Insert permissions for full privileges
INSERT INTO permissions (id, name, description)
VALUES
    (gen_random_uuid(), 'admin.create_user_role', 'Allows creating user-role assignments'),
    (gen_random_uuid(), 'admin.delete_user_role', 'Allows deleting user-role assignments'),
    (gen_random_uuid(), 'admin.view_user_role', 'Allows viewing user-role assignments'),
    (gen_random_uuid(), 'admin.create_role', 'Allows creating roles'),
    (gen_random_uuid(), 'admin.delete_role', 'Allows deleting roles'),
    (gen_random_uuid(), 'admin.create_permission', 'Allows creating permissions'),
    (gen_random_uuid(), 'admin.delete_permission', 'Allows deleting permissions'),
    (gen_random_uuid(), 'admin.create_user', 'Allows creating users'),
    (gen_random_uuid(), 'admin.delete_user', 'Allows deleting users')
ON CONFLICT ON CONSTRAINT permissions_name_key DO NOTHING;

-- Assign permissions to admin role
INSERT INTO role_permissions (role_id, permission_id)
SELECT
    '223e4567-e89b-12d3-a456-426614174001',
    id
FROM permissions
WHERE name IN (
    'admin.create_user_role',
    'admin.delete_user_role',
    'admin.view_user_role',
    'admin.create_role',
    'admin.delete_role',
    'admin.create_permission',
    'admin.delete_permission',
    'admin.create_user',
    'admin.delete_user'
);

-- Assign admin role to user
INSERT INTO user_roles (user_id, role_id)
VALUES (
    '4d3f3b89-4fc3-4db8-995d-0d27ceac520e',
    '223e4567-e89b-12d3-a456-426614174001'
);
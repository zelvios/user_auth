CREATE TABLE permissions
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE CHECK (name ~* '^[a-z_]+$'
) ,
    description VARCHAR(255)
);

INSERT INTO permissions (name, description)
VALUES
-- GET
('can_view_permission_table', 'See Table Permissions.'),                                -- bitmask: 1 << (0) = 1
('can_view_user_table', 'See Table Users.'),                                            -- bitmask: 1 << (1) = 2
('can_view_role_table', 'See Table Roles.'),                                            -- bitmask: 1 << (2) = 4
('can_view_permissions', 'View all permissions — cannot see full details.'),            -- bitmask: 1 << (3) = 8

-- POST
('can_create_role', 'Create new roles.'),                                              -- bitmask: 1 << (4) = 16
('can_create_permission', 'Create a new permission.'),                                 -- bitmask: 1 << (5) = 32
('can_assign_role', 'Assign roles to users.'),                                         -- bitmask: 1 << (6) = 64
('can_assign_permission', 'Assign a permission to a role.'),                           -- bitmask: 1 << (7) = 128
('can_take_admin', 'Give an Admin the user role.'),                                    -- bitmask: 1 << (8) = 256
('can_give_admin', 'Give admin to a user.'),                                           -- bitmask: 1 << (9) = 512

-- PUT
('can_suspend_user', 'Temporarily disable user access.'),                              -- bitmask: 1 << (10) = 1024
('can_reset_user_password', 'Reset another users password.'),                          -- bitmask: 1 << (11) = 2048
('can_force_logout_user', 'Log out a user remotely.'),                                 -- bitmask: 1 << (12) = 4096
('can_lock_system', 'Put system into maintenance mode.'),                              -- bitmask: 1 << (13) = 8192
('can_unlock_system', 'Resume system from maintenance mode.'),                         -- bitmask: 1 << (14) = 16384
('can_remove_permission', 'Remove permission from a role.'),                           -- bitmask: 1 << (15) = 32768
('can_remove_role', 'Remove roles from users.'),                                       -- bitmask: 1 << (16) = 65536

-- DELETE
('can_delete_role_user', 'Able to delete role User.'),                                 -- bitmask: 1 << (17) = 131072
('can_delete_role_admin', 'Able to delete role Admin.'),                               -- bitmask: 1 << (18) = 262144
('can_delete_admin', 'Delete an admin account.'),                                      -- bitmask: 1 << (19) = 524288
('can_delete_any_user', 'Delete any normal user account.');                            -- bitmask: 1 << (20) = 1048576
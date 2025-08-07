# Access Control: role and permission mapping.

This document outlines which roles have access to which permissions in the system.

---

## Role Bit Values

| Role        | Bit Value (Decimal) | Bit Value (Binary) |
|-------------|---------------------|--------------------|
| 👤 User      | 1                   | 0001               |
| 👥 Admin     | 2                   | 0010               |
| 💻 Developer | 4                   | 0100               |
| 🛡 Owner     | 8                   | 1000               |

*Roles can be combined by bitwise OR operations. For example, a user with roles User and Admin would have a role bitmask of `3` (1 + 2).*

---
---

## Roles and Their Permissions

### 👤 User
- _no special permission_

---

### 👥 Admin
- `can_view_permissions`
- `can_view_user_table`
- `can_view_role_table`
- `can_assign_role`
- `can_force_logout_user`
- `can_suspend_user`
- `can_reset_user_password`

---

### 💻 Developer
- `can_assign_permission`
- `can_view_permission_table`
- `can_remove_permission`
- `can_delete_permission`
- `can_delete_role_user`
- `can_delete_role_admin`
- `can_create_role`
- `can_create_permission`
- `can_delete_admin`
- `can_unlock_system`
- `can_lock_system`
- `can_remove_role`
- _and all admin permissions_
---

### 🛡 Owner
- access to all permissions:
- `can_take_admin`
- `can_give_admin`
- `can_assign_role`
- _and all developer and admin permissions_

---

## Notes

- Permissions follow the pattern `can_<action>_<target>`.
- The `name` field in `permissions` table is validated with regex: `^[a-z_]+$` (case-insensitive).

## User-Specific Permissions Bitmask

In addition to role-based permissions individual users can have **special permissions** assigned directly to them using a `permissions` bitmask field on the user record.

- The `permissions` integer stores additional permission bits granted specifically to the user.
- When checking permissions for a user **both the permissions granted by their roles and the user specific permissions are combined** using bitwise.
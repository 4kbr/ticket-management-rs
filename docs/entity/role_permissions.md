# **role_permissions**

Pivot table relasi many-to-many antara role dan permission.

## Fields

- `id`: integer, auto increment, **PK**

- `role_id`: integer, not null

  > FK ke `roles.id`

- `permission_id`: integer, not null

  > FK ke `permissions.id`

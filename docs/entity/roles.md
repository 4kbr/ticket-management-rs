# **roles**

Menyimpan daftar role dalam sistem.

## Fields

- `id`: integer, auto increment, **PK**

- `name`: string, not null, unique

  > Contoh: `SUPER_ADMIN`, `ADMIN`, `USER`

- `description`: string, nullable

- `created_at`: timestamp, not null

- `updated_at`: timestamp, not null

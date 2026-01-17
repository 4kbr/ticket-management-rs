# **permissions**

Menyimpan daftar permission granular.

## Fields

- `id`: integer, auto increment, **PK**

- `name`: string, not null, unique

  > Contoh: `GET_USERS`, `CREATE_USER`, `SUSPEND_USER`

- `description`: string, nullable

- `created_at`: timestamp

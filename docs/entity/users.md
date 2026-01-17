# **users**

Menyimpan data utama user.

## Fields

- `id`: integer, auto increment, **PK**, not null

  > Digunakan untuk relasi antar table (foreign key), **tidak pernah diekspos ke client**

- `secure_id`: UUID, not null, unique

  > ID publik yang digunakan di DTO dan API

- `name`: string, not null

  > Nama lengkap user

- `email`: string, not null, unique

  > Email user (digunakan untuk login)

- `password_hash`: string, not null,

  > Hasil hashing password (rekomendasi menggunakan bcrypt)

- `role_id`: integer, not null

  > Foreign key ke table `roles`

- `suspended_at`: timestamp, nullable

  > Jika tidak `NULL`, user dalam status suspended

- `suspended_reason`: string, nullable

  > Alasan user disuspend

- `suspended_by`: integer, nullable

  > FK ke `users.id` (admin yang melakukan suspend)

- `created_at`: timestamp, read-only, not null

  > tanggal user ini di create

- `updated_at`: timestamp, not null

  > tanggal data user ini terakhir kali di update

## Role Relations

```txt
users
 ├─ role_id → roles
 ├─ id → user_quotas.user_id
 ├─ id → quota_transactions.user_id
 ├─ id → quota_requests.user_id
 └─ id → tickets.user_id

roles
 └─ id → role_permissions.role_id

permissions
 └─ id → role_permissions.permission_id
```

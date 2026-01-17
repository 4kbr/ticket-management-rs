# **quota_requests** (Opsional – jika ada approval flow)

Menyimpan permintaan quota dari user.

## Fields

- `id`: integer, auto increment, **PK**

- `secure_id`: UUID, not null, unique

- `user_id`: integer, not null

  > FK ke `users.id`

- `requested_amount`: integer, not null

- `reason`: string, nullable

- `status`: enum, not null

  > `PENDING`, `APPROVED`, `REJECTED`, `CANCELED`

- `review_by`: integer, nullable

  > FK ke `users.id`, user yang reject / approved request ini

- `review_at`: timestamp, nullable

- `created_at`: timestamp

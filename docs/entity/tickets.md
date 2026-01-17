# **tickets**

- Menyimpan permintaan user
- Mengontrol lifecycle ticket
- Mengikat quota usage

## Fields

- `id`: integer, auto increment, **PK**, not null

  > ID internal, hanya untuk FK

- `secure_id`: UUID, not null, unique

  > ID publik yang dipakai di API

- `user_id`: integer, not null

  > FK ke `users.id`, pemilik ticket

- `assigned_admin_id`: integer, nullable

  > FK ke `users.id`, admin yang menangani ticket

- `title`: string, not null

  > Judul singkat ticket

- `description`: text, not null

  > Deskripsi detail masalah / permintaan

- `urgency_id`: int (FK -> ticket_urgencies.id)

- `quota_used`: integer, not null

  > Quota yang dikonsumsi saat ticket dibuat (snapshot)

- `status`: enum, not null

  > `INCOMING`, `ON_PROGRESS`, `DONE`, `CANCELED`, `REJECTED`

- `closed_at`: timestamp, nullable

  > Waktu ticket ditutup (DONE / CANCELED / REJECTED)

- `closed_by`: integer, nullable

  > FK ke `users.id`, admin yang menutup ticket

- `rejected_reason`: text, nullable

  > Dipakai untuk audit & transparansi ke user

- `canceled_reason`: text, nullable

  > Dipakai untuk audit & transparansi ke user

- `created_at`: timestamp, not null
- `updated_at`: timestamp, not null

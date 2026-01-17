# **quota_transactions**

Menyimpan **history perubahan quota** (ledger).

## Fields

- `id`: integer, auto increment, **PK**

- `secure_id`: UUID, not null, unique

  > ID publik transaksi quota

- `user_id`: integer, not null

  > FK ke `users.id`

- `ticket_id`: integer, nullable

  > FK ke `tickets.id` (jika transaksi berasal dari ticket)

- `type`: enum, not null

  > Contoh:
  > `INITIAL`, `TICKET_CREATE`, `TICKET_DONE`, `TICKET_CANCELED`, `QUOTA_REQUEST_APPROVED`

- `direction`: enum, not null

  > `IN` atau `OUT`

- `amount`: integer, not null

  > Jumlah quota yang berubah

- `reason`: string, nullable

  > Keterangan transaksi

- `created_at`: timestamp, not null

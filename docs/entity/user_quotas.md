# **user_quotas**

Menyimpan **quota aktif** milik user (state saat ini).

## Fields

- `id`: integer, auto increment, **PK**

- `user_id`: integer, not null

  > FK ke `users.id`

- `total_quota`: integer, not null

  > Total quota yang dimiliki user

- `used_quota`: integer, not null

  > Quota yang sedang digunakan

- `available_quota`: integer, not null

  > `total_quota - used_quota` (boleh disimpan demi performa)

- `updated_at`: timestamp

# **ticket_urgencies**

- Single source of truth untuk urgency
- Mengatur quota cost & rule bisnis

## fields

- `id`: integer, auto increment, **PK**, not null

  > ID internal, hanya untuk FK

- `code`: string, unique, not null

  > contoh: `NORMAL`, `HIGH`, `CRITICAL`

- `name`: string, not null

  > contoh: Normal, High, Critical

- `quota_cost`: int, not null

  > quota yang dipakai

- `description`: string, nullable

- `is_active`: boolean

  > untuk filter urgency

- `created_at`: timestamp, not null

- `updated_at`: timestamp, not null

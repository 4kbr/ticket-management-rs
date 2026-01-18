# Quota Transactions API

Dokumentasi untuk endpoint **riwayat transaksi quota** pada base URL `{base_url}/quota-transactions`.

- **Base path**: `/quota-transactions`
- **Tujuan**:
  - Menampilkan **history perubahan quota**
  - Audit & tracking penggunaan quota

- **Catatan penting**:
  - **Tidak ada endpoint create / update / delete**
  - Data dibuat **oleh sistem**, bukan user/admin secara langsung

## **GET `/quota-transactions`** : Mendapatkan daftar transaksi quota

- **Allowed Permissions**:
  - `ALL`,
  - `GET_QUOTA_TRANSACTIONS`

> Jika user **tidak memiliki permission** ini, maka:
>
> - hanya bisa melihat **transaksi miliknya sendiri**

### **Request**

- **Headers**: `Content-Type: application/json`

- **Query Parameters**:
  - `current_page` (integer, optional, min: 1)
  - `limit` (integer, optional, min: 1, default: 10)
  - `user_secure_id` (string, optional, admin only)
  - `type` (array of string, optional), opsi:
    - `INITIAL`
    - `TICKET_USE`
    - `TICKET_REFUND`
    - `QUOTA_REQUEST_APPROVED`
  - `direction` (string, optional) — `IN` | `OUT`
  - `created_start` (string, optional, format: `YYYY-MM-DD`)
  - `created_end` (string, optional, format: `YYYY-MM-DD`)
  - `sort_by` (string, optional, default: `created_at`, opsi:`created_at`, `amount`)
  - `sort_order` (string, optional, default: `desc`, opsi: `asc`, `desc`)

- **Bash**

```bash
curl -X GET "{base_url}/quota-transactions?current_page=1&limit=10&type=TICKET_USE&direction=OUT" \
  -H "Content-Type: application/json"
```

### **Business Logic**

1. **Scope data**
   - Admin (punya permission) → bisa filter semua user
   - User biasa → **hanya transaksi miliknya sendiri**

2. **Immutable**
   - Data transaksi **tidak boleh diubah**
   - Tidak ada soft delete

3. **Ordering**
   - Default urut berdasarkan `created_at desc`

### **Responses**

#### `200 OK` — Berhasil mendapatkan daftar transaksi

```json
{
  "status": "success",
  "message": "Quota transactions retrieved",
  "data": {
    "results": [
      {
        "secure_id": "uuid",
        "type": "TICKET_USE",
        "direction": "OUT",
        "amount": 2,
        "created_at": "2024-01-10T10:00:00Z",
        "user": {
          "secure_id": "user-uuid",
          "email": "user@example.com"
        },
        "ticket": {
          "secure_id": "ticket-uuid"
        }
      }
    ],
    "pagination": {
      "total_items": 100,
      "total_pages": 10,
      "current_page": 1,
      "limit": 10
    }
  }
}
```

#### `400 Bad Request` — Query parameter tidak valid

```json
{
  "status": "error",
  "message": "Query parameters invalid",
  "errors": {
    "type": "VALIDATION_ERROR",
    "fields": {
      "direction": "direction harus IN atau OUT",
      "type": "type tidak dikenali"
    }
  }
}
```

## **GET `/quota-transactions/{secure_id}`** : Detail transaksi quota

- **Allowed Permissions**:
  - `ALL`
  - `GET_QUOTA_TRANSACTION_DETAIL`
  - atau pemilik transaksi tersebut

### **Request**

- **Headers**:
  - `Content-Type: application/json`
  - `Authorization: Bearer {token}`

- **Path Parameters**:
  - `secure_id` (string, required)

- **Bash**

```bash
curl -X GET "{base_url}/quota-transactions/{secure_id}" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer {token}"
```

### **Business Logic**

1. **Authorization**
   - Admin → boleh akses semua
   - User → hanya jika transaksi miliknya

2. **Read-only**
   - Tidak ada perubahan data

### **Responses**

#### `200 OK` — Detail transaksi berhasil didapatkan

```json
{
  "status": "success",
  "message": "Quota transaction detail retrieved",
  "data": {
    "quota_transaction": {
      "secure_id": "uuid",
      "type": "QUOTA_REQUEST_APPROVED",
      "direction": "IN",
      "amount": 10,
      "reason": "Approved quota request",
      "created_at": "2024-01-02T12:00:00Z",
      "user": {
        "secure_id": "user-uuid",
        "email": "user@example.com"
      }
    }
  }
}
```

#### `404 Not Found` — Transaksi tidak ditemukan

```json
{
  "status": "error",
  "message": "Quota transaction not found",
  "errors": {
    "type": "NOT_FOUND"
  }
}
```

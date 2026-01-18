# Tickets API

Dokumentasi untuk endpoint **manajemen ticket** pada base URL `{base_url}/tickets`.

- **Base path**: `/tickets`
- **Tujuan**:
  - User membuat & memantau ticket
  - Admin memproses ticket
  - Sistem mengelola quota berdasarkan status ticket

## Business rules khusus

Jika user memiliki permission ALL:

- Tidak cek available_quota
- Tidak potong quota
- Tidak buat quota transaction

| Kondisi                | Cek                   |
| ---------------------- | --------------------- |
| Boleh create ticket    | `CREATE_TICKET`,`ALL` |
| Perlu quota?           | **TIDAK jika `ALL`**  |
| Potong quota           | **TIDAK jika `ALL`**  |
| Buat quota transaction | **TIDAK jika `ALL`**  |

## **POST `/tickets`** : Membuat ticket baru

- **Allowed Permissions**:
  - `ALL`
  - `CREATE_TICKET`

> Jika user **tidak memiliki permission**, maka request ditolak.

### **Request**

- **Headers**: `Content-Type: application/json`

- **Request Body**:

```json
{
  "title": "Tidak bisa login", // required, string, min: 3, max: 50
  "description": "Saya tidak bisa login sejak pagi", // required, string, min: 10, max: 255
  "urgency_secure_id": "uuid" // required, string, valid urgency secure_id
}
```

- **Bash**

```bash
curl -X POST "{base_url}/tickets" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Tidak bisa login",
    "description": "Saya tidak bisa login sejak pagi",
    "urgency_secure_id": "uuid"
  }'
```

### **Business Logic**

1. **Validasi**
   - `urgency_secure_id` harus valid & aktif
   - User **tidak boleh suspended**
   - `available_quota` user **harus >= quota_cost urgency**

2. **Atomic operation (WAJIB TRANSACTION)**
   - Jika salah satu gagal → **rollback semua**
   - Lakukan agregasi berikut:

   **a. Create ticket**

   ```txt
   status: INCOMING
   ```

   **b. Update quota user**

   ```txt
   user_quotas.available_quota -= urgency.quota_cost
   user_quotas.used_quota += urgency.quota_cost
   ```

   **c. Buat quota transaction**

   ```txt
   type: TICKET_USE
   direction: OUT
   amount: urgency.quota_cost
   ```

3. **Ownership**
   - `ticket.user_id = authenticated user`

### **Responses**

#### `201 Created` — Ticket berhasil dibuat

```json
{
  "status": "success",
  "message": "Ticket created",
  "data": {
    "ticket": {
      "secure_id": "uuid",
      "title": "Tidak bisa login",
      "status": "INCOMING",
      "urgency": {
        "id": "uuid",
        "name": "HIGH"
      },
      "created_at": "2024-01-10T10:00:00Z"
    }
  }
}
```

#### `400 Bad Request` — Quota tidak cukup

```json
{
  "status": "error",
  "message": "Insufficient quota",
  "errors": {
    "type": "BUSINESS_RULE_VIOLATION"
  }
}
```

## **GET `/tickets`** : Mendapatkan daftar ticket

- **Allowed Permissions**:
  - `ALL`
  - `GET_TICKETS`

> Jika user **tidak memiliki permission**, maka:
>
> - hanya bisa melihat **ticket miliknya sendiri**

### **Request**

- **Headers**: `Content-Type: application/json`

- **Query Parameters**:
  - `current_page` (integer, optional)
  - `limit` (integer, optional)
  - `status` (array of string, optional)
    - `INCOMING`, `ON_PROGRESS`, `DONE`, `CANCELED`, `REJECTED`

  - `urgency` (array of string, optional)
  - `user_secure_id` (string, optional, admin only)
  - `created_start` (string, optional)
  - `created_end` (string, optional)
  - `sort_by` (string, optional, default: `created_at`, opsi: `created_at`, `urgency`)
  - `sort_order` (string, optional, default: `desc`)

- **Bash**

```bash
curl -X GET "{base_url}/tickets?status=INCOMING&status=ON_PROGRESS" \
  -H "Content-Type: application/json"
```

### **Business Logic**

1. **Scope data**
   - Admin → bisa lihat semua ticket
   - User → hanya ticket miliknya

2. **Default sorting**
   - `created_at desc`

### **Responses**

#### `200 OK`

```json
{
  "status": "success",
  "message": "Tickets retrieved",
  "data": {
    "results": [
      {
        "secure_id": "uuid",
        "title": "Tidak bisa login",
        "status": "INCOMING",
        "urgency": {
          "name": "HIGH"
        },
        "user": {
          "secure_id": "user-uuid",
          "email": "user@example.com"
        },
        "created_at": "2024-01-10T10:00:00Z"
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

## **GET `/tickets/{secure_id}`** : Detail ticket

- **Allowed Permissions**:
  - `ALL`
  - `GET_TICKET_DETAIL`
  - atau pemilik ticket

### **Request**

- **Headers**: `Content-Type: application/json`

- **Path Parameters**:
  - `secure_id` (string, required)

- **Bash**

```bash
curl -X GET "{base_url}/tickets/{secure_id}" \
  -H "Content-Type: application/json"
```

### **Business Logic**

1. **Authorization**
   - Admin → boleh akses semua
   - User → hanya ticket miliknya

### **Responses**

#### `200 OK`

```json
{
  "status": "success",
  "data": {
    "ticket": {
      "secure_id": "uuid",
      "title": "Tidak bisa login",
      "description": "...",
      "status": "ON_PROGRESS",
      "urgency": {
        "name": "HIGH",
        "quota_cost": 2
      },
      "user": {
        "secure_id": "user-uuid",
        "email": "user@example.com"
      },
      "created_at": "2024-01-10T10:00:00Z",
      "updated_at": "2024-01-11T10:00:00Z"
    }
  }
}
```

## **PATCH `/tickets/{secure_id}/status`** : Update status ticket (Admin)

- **Allowed Permissions**:
  - `ALL`
  - `UPDATE_TICKET_STATUS`

### **Request**

- **Headers**: `Content-Type: application/json`

- **Request Body**:

```json
{
  "status": "DONE",
  "note": "Issue resolved"
}
```

- **Bash**

```bash
curl -X PATCH "{base_url}/tickets/{secure_id}/status" \
  -H "Content-Type: application/json" \
  -d '{"status":"DONE"}'
```

### **Business Logic**

1. **Validasi transisi status**
   - `INCOMING → ON_PROGRESS`
   - `ON_PROGRESS → DONE | CANCELED | REJECTED`

2. **Jika status akhir (`DONE`, `CANCELED`, `REJECTED`)**

   **Atomic operation (WAJIB TRANSACTION)**
   - Refund quota user:

     ```txt
     user_quotas.available_quota += urgency.quota_cost
     user_quotas.used_quota -= urgency.quota_cost
     ```

   - Buat quota transaction:

     ```txt
     type: TICKET_REFUND
     direction: IN
     amount: urgency.quota_cost
     ```

   - Update ticket:

     ```txt
     status, updated_at, reviewed_by
     ```

3. **Jika gagal salah satu → rollback semua**

### **Responses**

#### `200 OK`

```json
{
  "status": "success",
  "message": "Ticket status updated"
}
```

## Penutup (Catatan Penting)

- Semua perubahan quota **harus tercatat di quota_transactions**
- Semua operasi quota **harus transactional**
- Tidak boleh ada perubahan manual tanpa history

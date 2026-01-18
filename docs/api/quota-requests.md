# Quota Requests API

Dokumentasi untuk endpoint permintaan quota pada base URL `{base_url}/quota-requests`.

- **Base path**: `/quota-requests`
- **Tujuan**:
  - `User` mengajukan permintaan quota
  - `Admin` melakukan approval / rejection
  - Sistem menjaga konsistensi quota & audit

## **POST `/quota-requests`** : Membuat quota request

Digunakan oleh:

- **User** → request quota (PENDING)

- **Admin** → grant quota ke user (auto APPROVED)

- **Allowed Permissions**:
  - `REQUEST_QUOTA` (user)
  - `ALL` (super admin)
  - `APPROVE_QUOTA_REQUEST` (admin)

### **Request**

- **Headers**: `Content-Type: application/json`
- **Request Body**:

```json
{
  "user_secure_id": "uuid", // required hanya untuk permission `APPROVE_QUOTA_REQUEST`, untuk `REQUEST_QUOTA` ini tidak boleh ada
  "requested_amount": 10, // required, integer > 0
  "reason": "Butuh quota tambahan" // optional, alasan request
}
```

> ⚠️ `user_secure_id`:
>
> - **tidak boleh** diisi oleh user biasa
> - **wajib** diisi jika admin membuat request untuk user lain

- **Bash**

```bash
curl -X POST {base_url}/quota-requests \
  -H "Content-Type: application/json" \
  -d '{"requested_amount":10,"reason":"Butuh quota tambahan"}'
```

### **Business Logic**

1. **REQUEST_QUOTA**: yang bisa request quota untuk diri sendiri hanyalah role yang punya permission `REQUEST_QUOTA`

2. **Jika user biasa**
   - `user_secure_id` diambil dari token
   - `status` = `PENDING`

3. **Jika admin**
   - `status` = `APPROVED`
   - `reviewed_by` = admin
   - **aggregation atau berbarengan dengan pembuatan quota_transaction dan update di user_quota**

4. **Validasi**
   - `requested_amount > 0`
   - user tidak boleh membuat request baru jika masih ada `PENDING`

### **Responses**

#### `201 Created` — Quota request berhasil dibuat

```json
{
  "status": "success",
  "message": "Quota request created",
  "data": {
    "quota_request": {
      "secure_id": "uuid",
      "requested_amount": 10,
      "status": "PENDING",
      "created_at": "2024-01-01T12:00:00Z"
    }
  }
}
```

#### `400 Bad Request` — Data tidak valid

```json
{
  "status": "error",
  "message": "Request body invalid",
  "errors": {
    "type": "VALIDATION_ERROR",
    "fields": {
      "requested_amount": "Requested amount harus lebih dari 0", // misal
      "user_secure_id": "Tujuan pemberian quota wajib di isi" // atau
    }
  }
}
```

## **GET `/quota-requests`** : Mendapatkan daftar quota request

- **Allowed Permissions**:
  - `ALL`
  - `GET_QUOTA_REQUESTS`

> Tanpa permission ini, user **hanya melihat miliknya sendiri**

### **Request**

- **Query Parameters**:
  - `status` (array of string, optional) — `PENDING`, `APPROVED`, `REJECTED`, `CANCELED`
  - `user_secure_id` (string, optional, allowed permission only)
  - `current_page` (integer, optional, min: 1)
  - `limit` (integer, optional, min: 1, default: 10)
  - `created_start` (date iso string `YYYY-MM-DD`, optional)
  - `created_end` (date iso string `YYYY-MM-DD`, optional)
  - `sort_by` (string, default: `created_at`, `request_amount`)
  - `sort_order` (string, default: `desc`)

- **Bash**

```bash
curl -X GET "{base_url}/quota-requests?status=PENDING&current_page=1&limit=10" \
  -H "Content-Type: application/json"
```

### **Responses**

#### `200 OK`

```json
{
  "status": "success",
  "message": "Quota requests retrieved",
  "data": {
    "results": [
      {
        "secure_id": "uuid",
        "requested_amount": 10,
        "status": "PENDING",
        "created_at": "2024-01-01T12:00:00Z",
        "user_secure_id": "uuid",
        "user_email": "user@email.com" // diambil dari table users
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

## **GET `/quota-requests/{secure_id}`** : Detail quota request

- **Allowed Permissions**:
  - `ALL`
  - `GET_QUOTA_REQUEST_DETAIL`
  - atau pemilik request itu sendiri

### **Responses**

- `200 OK`

```json
{
  "status": "success",
  "message": "Quota request detail retrieved",
  "data": {
    "quota_request": {
      "secure_id": "uuid",
      "requested_amount": 10,
      "reason": "Butuh quota tambahan",
      "status": "APPROVED",
      "reviewed_by": "admin-secure-id",
      "reviewed_at": "2024-01-02T10:00:00Z",
      "created_at": "2024-01-01T12:00:00Z",
      "user_secure_id": "uuid",
      "user_email": "user@example.com"
    }
  }
}
```

- `404 Not Found` — Quota request tidak ditemukan atau tidak punya akses

```json
{
  "status": "error",
  "message": "Quota request not found",
  "errors": {
    "type": "NOT_FOUND"
  }
}
```

## **PATCH `/quota-requests/{secure_id}/status`** : Update status quota request

- **Allowed Permissions**:
  - `ALL`
  - `APPROVE_QUOTA_REQUEST`

### **Request Body**

```json
{
  "status": "APPROVED", // APPROVED | REJECTED | CANCELED
  "note": "Approved by admin"
}
```

### **Business Logic**

#### 1️⃣ APPROVED

- hanya boleh jika status saat ini `PENDING`
- lakuan aggregasi:
  - buat **quota transaction**
    ```txt
    type: QUOTA_REQUEST_APPROVED
    direction: IN
    amount: requested_amount
    ```
  - update `user_quotas.total_quota` dan `user_quotas.available_quota`
  - set `reviewed_by`, `reviewed_at`, `note` , status ke `APPROVED`

#### 2️⃣ REJECTED

- hanya boleh jika status `PENDING`
- **TIDAK ADA** perubahan quota
- set `reviewed_by`, `reviewed_at`, `note`, status ke `REJECTED`

#### 3️⃣ CANCELED

- hanya boleh oleh **pemilik request**
- hanya jika status `PENDING`
- **TIDAK ADA** perubahan quota
- set status ke `CANCELED`

### **Responses**

#### `200 OK`

```json
{
  "status": "success",
  "message": "Quota request approved"
}
```

#### `409 Conflict` — Status tidak valid

```json
{
  "status": "error",
  "message": "Quota request status cannot be changed",
  "errors": {
    "type": "BUSINESS_RULE_VIOLATION"
  }
}
```

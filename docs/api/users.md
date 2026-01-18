# Users API

Dokumentasi untuk endpoint otentikasi pada base URL `{base_url}/users`.

- **Base path**: `/users`
- **Tujuan**: Manajemen data user

---

## **POST `/users`** : Super Admin Menambah user baru.

- **Allowed Permissions**: `ALL`, `CREATE_USER`

### **Request**

- **Headers**: `Content-Type: application/json`
- **Request body** (JSON):

```json
{
  "name": "Nama Pengguna", // required, string
  "email": "user@example.com", // required, string (format email), unique
  "role_id": 1 // required, integer (berdasarkan dropdown role yang tersedia)
}
```

- **Bash**

```bash
curl -X POST {base_url}/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Nama Pengguna","email":"user@example.com","role_id":1}'
```

### **Responses**:

- `201 Created` — Pembuatan user berhasil. Contoh body:

```json
{
  "status": "success",
  "message": "User created",
  "data": {
    "user": {
      "secure_id": "uuid",
      "name": "Nama Pengguna",
      "email": "user@example.com",
      "role": "USER"
    }
  }
}
```

- `400 Bad Request` — Data tidak valid (mis. email sudah terpakai, password terlalu pendek).

```json
{
  "status": "error",
  "message": "Request body invalid",
  "errors": {
    "type": "VALIDATION_ERROR",
    "fields": {
      "email": "Email sudah terpakai",
      "role_id": "Role tidak valid"
    }
  }
}
```

- `403 Forbidden` — User tidak memiliki izin untuk membuat user baru.

```json
{
  "status": "error",
  "message": "Permission denied",
  "errors": {
    "type": "AUTHORIZATION_ERROR"
  }
}
```

---

## **GET `/users`** : Super Admin Mendapatkan daftar semua user.

- **Allowed Permissions**: `ALL`, `GET_USERS`

### **Request**

- **Headers**: `Content-Type: application/json`
- **Query Parameters**:
  - `current_page` (integer, optional, min: 1) — Nomor halaman untuk pagination.
  - `limit` (integer, optional, min:1, default: 10) — Jumlah item per halaman.
  - `secure_id` (string, optional) — Filter user berdasarkan secure_id.
  - `email` (string, optional) — Filter user berdasarkan email (pencarian partial).
  - `name` (string, optional) — Filter user berdasarkan nama (pencarian partial).
  - `roles` (array of strings, optional) — Filter user berdasarkan role (mis. `ADMIN`, `USER`).
  - `suspended` (boolean, optional) — Filter user berdasarkan status suspended (`true` atau `false`).
  - `created_start` (string, optional, format: `YYYY-MM-DD`) — Filter user yang dibuat setelah tanggal ini.
  - `created_end` (string, optional, format: `YYYY-MM-DD`) — Filter user yang dibuat sebelum tanggal ini.
  - `updated_start` (string, optional, format: `YYYY-MM-DD`) — Filter user yang diupdate setelah tanggal ini.
  - `updated_end` (string, optional, format: `YYYY-MM-DD`) — Filter user yang diupdate sebelum tanggal ini.
  - `sort_by` (string, optional, default: `created_at`) — Field untuk mengurutkan hasil (mis. `name`, `email`, `created_at`, `updated_at`,`available_quota`, `used_quota`).
  - `sort_order` (string, optional, default: `desc`) — Urutan sorting, bisa `asc` atau `desc`.

- **Bash**

```bash
curl -X GET "{base_url}/users?current_page=1&limit=10&name=John&roles=ADMIN&roles=USER&sort_by=name&sort_order=asc" \
  -H "Content-Type: application/json"
```

### **Responses**:

- `200 OK` — Mendapatkan daftar user berhasil. Contoh body:

```json
{
  "status": "success",
  "message": "Users retrieved",
  "data": {
    "results": [
      {
        "secure_id": "uuid",
        "name": "Nama Pengguna",
        "email": "user@example.com",
        "role": "USER",
        "suspended": false,
        "available_quota": 100,
        "used_quota": 50,
        "created_at": "2024-01-01T12:00:000Z",
        "updated_at": "2024-01-10T12:00:000Z"
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

- `400 Bad Request` — Query parameters tidak valid.

```json
{
  "status": "error",
  "message": "Query parameters invalid",
  "errors": {
    "type": "VALIDATION_ERROR",
    "fields": {
      "current_page": "current_page harus >= 1",
      "sort_order": "sort_order harus 'asc' atau 'desc'",
      "query_lain": "query_lain tidak valid"
    }
  }
}
```

---

## **GET `/users/{secure_id}`** : Super Admin Mendapatkan detail user berdasarkan secure_id.

- **Allowed Permissions**: `ALL`, `GET_USER_DETAIL`
  role dengan permission GET_USER_DETAIL tidak bisa mengakses detail super admin, data super admin hanya bisa dilihat oleh permission ALL.

### **Request**

- **Headers**: `Content-Type: application/json`
- **Path Parameters**:
  - `secure_id` (string, required) — secure_id user yang ingin diambil detailnya.

- **Bash**

```bash
curl -X GET "{base_url}/users/{secure_id}" \
  -H "Content-Type: application/json"
```

### **Responses**:

- `200 OK` — Mendapatkan detail user berhasil. Contoh body:

```json
{
  "status": "success",
  "message": "User detail retrieved",
  "data": {
    "user": {
      "secure_id": "uuid",
      "name": "Nama Pengguna",
      "email": "user@example.com",
      "role": "USER",
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-10T12:00:00Z",
      "quota": {
        "total_quota": 100,
        "used_quota": 50,
        "available_quota": 50
      },
      "suspended_at": null, // jika ada maka ini adalah timestamp saat user disuspend
      "suspended_reason": null, // jika ada maka ini adalah alasan user disuspend (string)
      "suspended_by": null, // jika ada maka ini adalah secure_id admin yang mensuspend user
      "ticket": {
        "total_tickets": 10,
        "open_tickets": 2,
        "closed_tickets": 8
      }
    }
  }
}
```

- `404 Not Found` — User dengan secure_id tersebut tidak ditemukan.

```json
{
  "status": "error",
  "message": "User not found",
  "errors": {
    "type": "NOT_FOUND"
  }
}
```

---

## **GET `/users/me`** : User Mendapatkan data dirinya sendiri.

- **Allowed Permissions**: _(Authenticated User)_

> Endpoint ini digunakan untuk mendapatkan data user yang sedang login.
> Tidak memerlukan permission admin.

### **Request**

- **Headers**:
  - `Content-Type: application/json`
  - `Authorization: Bearer <access_token>`

- **Bash**

```bash
curl -X GET {base_url}/users/me \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <access_token>"
```

### **Responses**

- `200 OK` — Berhasil mendapatkan data user.

```json
{
  "status": "success",
  "message": "User data retrieved",
  "data": {
    "user": {
      "secure_id": "uuid",
      "name": "Nama Pengguna",
      "email": "user@example.com",
      "role": "USER",
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-10T12:00:00Z",
      "quota": {
        "total_quota": 100,
        "used_quota": 50,
        "available_quota": 50
      },
      "ticket": {
        "total_tickets": 10,
        "open_tickets": 2,
        "closed_tickets": 8
      }
    }
  }
}
```

- `401 Unauthorized` — token hilang/invalid/expired.

```json
{
  "status": "error",
  "message": "Invalid or expired access token",
  "errors": {
    "type": "AUTHENTICATION_ERROR"
  }
}
```

- `403 Forbidden` — User suspended.

```json
{
  "status": "error",
  "message": "Your account is suspended",
  "errors": {
    "type": "USER_SUSPENDED"
  }
}
```

---

## **PATCH `/users/me`** : User Mengubah data dirinya sendiri.

- **Allowed Permissions**: _(Authenticated User)_

> Digunakan oleh user untuk mengubah data pribadinya sendiri.
> **Tidak** digunakan untuk admin mengubah user lain.

- User tidak bisa mengubah emailnya, hanya permission `ALL` atau `UPDATE_DATA_USER` yang bisa mengubah data email user.

### **Request**

- **Headers**:
  - `Content-Type: application/json`
  - `Authorization: Bearer <access_token>`

- **Request body** (JSON):

```json
{
  "name": "Nama Baru" // optional, string (min:3, max:100)
}
```

- **Bash**

```bash
curl -X PATCH {base_url}/users/me \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <access_token>" \
  -d '{"name":"Nama Baru"}'
```

### **Responses**

- `200 OK` — Data user berhasil diperbarui.

```json
{
  "status": "success",
  "message": "User data updated",
  "data": {
    "user": {
      "secure_id": "uuid",
      "name": "Nama Baru",
      "email": "baru@example.com"
    }
  }
}
```

- `400 Bad Request` — Data tidak valid.

```json
{
  "status": "error",
  "message": "Request body invalid",
  "errors": {
    "type": "VALIDATION_ERROR",
    "fields": {
      "name": "Nama harus antara 3-100 karakter"
    }
  }
}
```

---

## **PUT `/users/me/password`** : User Mengubah password dirinya sendiri.

- **Allowed Permissions**: _(Authenticated User)_
  > Digunakan oleh user untuk mengubah password pribadinya sendiri.

### **Request**

- **Headers**:
  - `Content-Type: application/json`
  - `Authorization: Bearer <access_token>`
- **Request body** (JSON):

```json
{
  "current_password": "password_lama", // required, string
  "new_password": "password_baru" // required, string (min:6)
}
```

- **Bash**

```bash
curl -X PUT {base_url}/users/me/password \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <access_token>" \
  -d '{"current_password":"password_lama","new_password":"password_baru"}'
```

### **Responses**

- `200 OK` — Password berhasil diubah.

```json
{
  "status": "success",
  "message": "Password updated"
}
```

- `400 Bad Request` — Data tidak valid atau password lama salah.

```json
{
  "status": "error",
  "message": "Request body invalid",
  "errors": {
    "type": "VALIDATION_ERROR",
    "fields": {
      "current_password": "Password lama salah",
      "new_password": "Password baru harus minimal 6 karakter"
    }
  }
}
```

---

## **PATCH `/users/{secure_id}`** : Admin Mengubah data user.

- **Allowed Permissions**: `ALL`, `UPDATE_DATA_USER`

> Digunakan oleh admin untuk mengubah **data identitas user lain** (selain status).

### **Request**

- **Headers**: `Content-Type: application/json`

- **Path Parameters**:
  - `secure_id` (string, required)

- **Request body** (JSON):

```json
{
  "name": "Nama Baru", // optional, string
  "email": "baru@example.com", // optional, string (format email, unique)
  "role_id": 2 // optional, integer
}
```

- **Bash**

```bash
curl -X PATCH {base_url}/users/{secure_id} \
  -H "Content-Type: application/json" \
  -d '{"name":"Nama Baru","email":"baru@example.com","role_id":2}'
```

### **Responses**

- `200 OK` — Data user berhasil diperbarui.

```json
{
  "status": "success",
  "message": "User updated",
  "data": {
    "user": {
      "secure_id": "uuid",
      "name": "Nama Baru",
      "email": "baru@example.com",
      "role": "ADMIN"
    }
  }
}
```

- `400 Bad Request` — Data tidak valid.

```json
{
  "status": "error",
  "message": "Request body invalid",
  "errors": {
    "type": "VALIDATION_ERROR",
    "fields": {
      "email": "Email sudah terpakai",
      "role_id": "Role tidak valid"
    }
  }
}
```

- `403 Forbidden` — Tidak memiliki izin.

```json
{
  "status": "error",
  "message": "Permission denied",
  "errors": {
    "type": "AUTHORIZATION_ERROR"
  }
}
```

# **User Status Management (Action-based)**

> Endpoint di bawah ini **KHUSUS** untuk perubahan status user.
> **Tidak digabung** dengan update data user.

---

## **PUT `/users/{secure_id}/suspend`** : Suspend user

- **Allowed Permissions**: `ALL`, `SUSPEND_USER`

### **Request**

- **Headers**: `Content-Type: application/json`
- **Request body**:

```json
{
  "reason": "Violation of policy" // required, string
}
```

- **Bash**

```bash
curl -X PUT {base_url}/users/{secure_id}/suspend \
  -H "Content-Type: application/json" \
  -d '{"reason":"Violation of policy"}'
```

### **Responses**

- `200 OK` — User berhasil disuspend.

```json
{
  "status": "success",
  "message": "User suspended"
}
```

---

## **PUT `/users/{secure_id}/reactivate`** : Mengaktifkan kembali user

- **Allowed Permissions**: `ALL`, `REACTIVATE_USER`

### **Request**

- **Headers**: `Content-Type: application/json`

- **Bash**

```bash
curl -X PUT {base_url}/users/{secure_id}/reactivate \
  -H "Content-Type: application/json"
```

### **Responses**

- `200 OK` — User berhasil diaktifkan kembali.

```json
{
  "status": "success",
  "message": "User reactivated"
}
```

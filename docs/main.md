# Main dokumentasi

## API Documentation

Semua dokumentasi yang berhubungan dengan API akan ada di folder docs/api

- default response success json:

```json
{
  "status": "success",
  "message": "Deskripsi sukses", // opsional
  "data": {} // berisi response spesifik endpoint, bisa objek, array, atau nilai primitif
}
```

- default response success pagination json:

```json
{
  "status": "success",
  "message": "Deskripsi sukses", // opsional
  "data": {
    "results": [], // array berisi item pada halaman ini
    "pagination": {
      "total_items": 100,
      "total_pages": 10,
      "current_page": 1,
      "limit": 10
    }
  }
}
```

- default response error json:

```json
{
  "status": "error",
  "message": "Deskripsi error",
  "errors": {
    "type": "VALIDATION_ERROR", // tipe error, mis. VALIDATION_ERROR, AUTHENTICATION_ERROR, INTERNAL_ERROR, dll.
    "fields": {
      "email": "Email sudah terpakai",
      "password": "Password terlalu pendek"
    }
  } // opsional, berisi informasi tambahan tentang error
}
```

- type error umum:
  - `VALIDATION_ERROR` — Data input tidak valid.
  - `AUTHENTICATION_ERROR` — Gagal autentikasi (mis. token tidak valid atau kadaluarsa).
  - `AUTHORIZATION_ERROR` — Akses ditolak (mis. user tidak punya izin).
  - `INTERNAL_ERROR` — Error server internal.
  - `NOT_FOUND` — Resource tidak ditemukan.
  - `USER_SUSPENDED` — User dinonaktifkan.

- Walaupun mungkin beberapa endpoint tidak ada example response yang lengkap, tetapi setiap endpoint mengikuti format response di atas. dan bisa saja mengembalikan error yang sudah dijelaskan.

- Semua endpoint yang membutuhkan autentikasi harus mengirim header `Authorization: Bearer {access_token}` kecuali disebutkan lain.

- Semua request dan response menggunakan format JSON kecuali disebutkan lain.

- Semua timestamp menggunakan format ISO 8601 dalam UTC: `YYYY-MM-DDTHH:MM:SSZ`.

- **Allowed Permissions** merupakan daftar izin yang diperlukan user untuk mengakses endpoint tersebut. Role user tersebut harus memiliki setidaknya satu dari izin yang tercantum agar dapat mengakses endpoint.

- Semua endpoint yang bersifat create, update, delete bisa saja membuat log aktivitas di sistem. Log ini hanya bisa diakses oleh admin melalui endpoint log aktivitas.

## Authentication

API ini menggunakan **Cookie-based Authentication**.

- Access token dan refresh token disimpan di cookie
- Cookie akan otomatis dikirim oleh browser pada setiap request
- Endpoint yang membutuhkan otentikasi akan memvalidasi cookie tersebut

Catatan:

- Untuk client non-browser, cookie harus dikirim manual
- Pastikan `withCredentials: true` untuk SPA

## Entity Documentation

Dokumentasi ini menjelaskan struktur entity (database level) yang digunakan dalam sistem.
Field internal (`id`) **tidak diekspos ke client**, sedangkan `secure_id` digunakan untuk komunikasi API.

# Auth API

Dokumentasi untuk endpoint otentikasi pada base URL `{base_url}/auth`.

- **Base path**: `/auth`
- **Tujuan**: Registrasi, login, dan refresh token via cookie

## **Endpoints**

### **POST `/auth/signup`**: Registrasi user baru.

#### **Request**

- **Headers**: `Content-Type: application/json`
- **Request body** (JSON):

```json
{
  "name": "Nama Pengguna", // required, string
  "email": "user@example.com", // required, string (format email), unique
  "password": "rahasia123" // required, string (min 6 karakter)
}
```

- **Bash**

```bash
curl -X POST {base_url}/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"name":"Nama Pengguna","email":"user@example.com","password":"rahasia123"}'
```

#### **Responses**:

- `201 Created` — Registrasi berhasil. Contoh body:

```json
{
  "status": "success",
  "message": "User registered",
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
  "message": "Email already in use",
  "errors": {
    "type": "VALIDATION_ERROR",
    "fields": {
      "email": "Email sudah terpakai",
      "password": "Password tidak memenuhi syarat"
    }
  }
}
```

### **POST `/auth/signin`**: Login dan penerbitan token.

#### **Request**

- **Headers**: `Content-Type: application/json`
- **Request body** (JSON):

```json
{
  "email": "user@example.com", // required, string (format email)
  "password": "rahasia123" // required, string
}
```

- **Bash**

```bash
curl -X POST {base_url}/auth/signin \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"rahasia123"}'
```

- **Behavior**: Jika kredensial valid, server mengembalikan `token` di body response dan meng-set dua cookie:
  - Cookie `access_token` (HttpOnly, Secure) — berisi access token (opsional jika juga dikirim di body).
  - Cookie `refresh_token` (HttpOnly, Secure, Path=/auth/refresh) — berisi refresh token untuk permintaan refresh berikutnya.

#### **Responses**:

- **Response contoh (200 OK)**:

```json
{
  "status": "success",
  "data": {
    "token": "eyJhbGciOi...", // JWT token untuk dipakai di header Authorization
    "token_type": "bearer",
    "expires_in_ms": 3600000 // dalam milidetik
  }
}
```

- `400 Bad Request` — data input tidak valid.

```json
{
  "status": "error",
  "message": "Invalid input data",
  "errors": {
    "type": "VALIDATION_ERROR",
    "fields": {
      "email": "Format email tidak valid"
    }
  }
}
```

- `401 Unauthorized` — kredensial salah.

```json
{
  "status": "error",
  "message": "Invalid email or password",
  "errors": {
    "type": "AUTHENTICATION_ERROR"
  }
}
```

- **Set-Cookie (contoh header, otomatis ter set)**:

```
Set-Cookie: access_token=<access_token>; HttpOnly; Secure; Path=/; Max-Age=3600; SameSite=Strict
Set-Cookie: refresh_token=<refresh-token>; HttpOnly; Secure; Path=/auth/refresh; Max-Age=1209600; SameSite=Strict
```

### **POST `/auth/refresh`**: Perbarui access token menggunakan refresh token yang tersimpan di cookie.

#### **Request**

- **Headers**: `Content-Type: application/json` (body kosong atau minimal)
- **Authentication**: Tidak mengirimkan token di body; server membaca `refresh_token` dari cookie.
- **Behavior**: Jika `refresh_token` valid, server mengeluarkan token baru dan mengupdate cookie `access_token`.

- **Bash**

```bash
curl -X POST {base_url}/auth/refresh \
  -H "Content-Type: application/json"
```

#### **Responses**

- **Response contoh (200 OK)**:

```json
{
  "token": "eyJhbGciOi...",
  "token_type": "bearer",
  "expires_in_ms": 3600000
}
```

- **Set-Cookie (contoh header, otomatis ter set)**:

```
Set-Cookie: access_token=<new-token>; HttpOnly; Secure; Path=/; Max-Age=3600; SameSite=Strict
```

- `401 Unauthorized` — refresh token hilang/invalid/expired.

```json
{
  "status": "error",
  "message": "Invalid or expired refresh token",
  "errors": {
    "type": "AUTHENTICATION_ERROR"
  }
}
```

## **Contoh penggunaan (curl)**

- Register:

```bash
curl -X POST {base_url}/auth/signup \
	-H "Content-Type: application/json" \
	-d '{"name":"Budi","email":"budi@example.com","password":"rahasia123"}'
```

- Signin (menangkap cookie di file untuk request selanjutnya):

```bash
curl -i -c cookies.txt -X POST {base_url}/auth/signin \
	-H "Content-Type: application/json" \
	-d '{"email":"budi@example.com","password":"rahasia123"}'
```

- `-c cookies.txt` menyimpan cookie (termasuk `refresh_token`).

- Refresh access token (menggunakan cookie tersimpan):

```bash
curl -i -b cookies.txt -c cookies.txt -X POST {base_url}/auth/refresh
```

- `-b cookies.txt` mengirim cookie, `-c cookies.txt` memperbarui cookie bila server mengirim Set-Cookie baru.

**Keamanan & Catatan implementasi**

- Simpan `access_token` dan `refresh_token` sebagai HttpOnly cookie untuk mencegah akses via JavaScript.
- Beri atribut `Secure` dan `SameSite=Strict`/`Lax` sesuai kebutuhan.
- Set `Path` untuk `refresh_token` ke `/auth/refresh` jika ingin membatasi pengiriman cookie hanya ke endpoint refresh.
- Pilih durasi `Max-Age` yang sesuai: access token singkat (mis. 1 jam), refresh token lebih panjang (mis. 14 hari).
- Terapkan mekanisme revocation/blacklist untuk refresh token jika perlu.

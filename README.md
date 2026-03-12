<div align="center">
  <a href="#">
    <img src="https://img.shields.io/badge/%E2%96%A3_LazyBucket-9333ea?style=for-the-badge&logoColor=white" alt="LazyBucket" height="40"/>
  </a>

  <p>Lightweight self-hosted object storage</p>

  ![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)
  ![React](https://img.shields.io/badge/React-20232a?style=flat-square&logo=react&logoColor=61dafb)
  ![Docker](https://img.shields.io/badge/Docker-2496ed?style=flat-square&logo=docker&logoColor=white)
</div>

---

> "The reason you all obey me is because you want someone stronger to follow.

A lightweight object storage inspired by the simplicity of S3.
Built to store, serve and scale files — without unnecessary complexity.

---

## Philosophy

In a world where systems grow bloated and slow, **LazyBucket** stands as a simple weapon.

It has one purpose:

> **Store objects. Serve them fast. Stay reliable.**

No unnecessary layers.
No heavy cloud dependency.
Just a clean and powerful storage core.

---

## Features

- Object storage with S3-like design
- Fast file serving with streaming
- Bucket-based organization
- Web UI for managing buckets and files
- Basic auth protected API
- Single binary — ships as one Docker container

---

Objects are stored directly on disk.
Metadata is indexed in SQLite for fast lookup.

---

## Getting Started

**Run with Docker:**

```bash
docker build -t lazybucket .

docker run -p 8000:8000 \
  -e USER_LOGIN=admin \
  -e USER_PASSWORD=secret \
  -v $(pwd)/database.db:/app/database.db \
  -v $(pwd)/storage:/app/storage \
  lazybucket
```

Open `http://localhost:8000` in your browser.

---

**Run locally:**

```bash
# Backend
USER_LOGIN=admin USER_PASSWORD=secret cargo run

# Frontend
cd frontend && npm install && npm run dev
```

---

## API

All API routes are prefixed with `/api`.

**Buckets**

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/buckets` | List buckets |
| `PUT` | `/api/buckets/:name` | Create bucket |
| `PATCH` | `/api/buckets/:name` | Rename bucket |
| `DELETE` | `/api/buckets/:name` | Delete bucket |

**Objects**

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/:bucket` | List objects |
| `PUT` | `/api/:bucket/*key` | Upload object |
| `GET` | `/api/:bucket/*key` | Download object |
| `PATCH` | `/api/:bucket/*key` | Rename object |
| `DELETE` | `/api/:bucket/*key` | Delete object |

All endpoints except `GET /:bucket/*key` require `Authorization: login:password` header.

---

## License

[MIT](./LICENSE) © 2026 [Tsunami43](https://github.com/Tsunami43)

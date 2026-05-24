# ca-api

Self-contained pure-Rust PKI / CA HTTP service. Generates CA roots, issues and revokes client certificates, maintains CRLs, and exports PKCS#12 bundles — all via `rcgen` + `openssl` FFI, no `easy-rsa` / `openssl` CLI required at runtime.


## Configuration

`ca-api` reads a single YAML settings file from `~/.ca-api` (i.e. `/root/.ca-api` in the default Docker container). It has one field:

```yaml
config_path: /root/pki
```

`config_path` is the on-disk root where all CA folders live (see [Storage layout](#storage-layout) below).


## Docker compose example

```yaml
services:
  ca-api:
    image: myjettools/ca-api:0.2.0
    hostname: ca-api
    container_name: ca-api
    restart: always
    deploy:
      resources:
        limits:
          memory: 64Mb
    volumes:
      # settings file (single YAML file — must exist on host before `up`)
      - ./ca-api:/root/.ca-api
      # PKI storage (directory — keeps CA roots, keys, CRLs, issued certs)
      - ./pki:/root/pki
    ports:
      - "5959:8000"
    logging:
      options:
        max-size: 512Kb
        max-file: 1
    networks:
      - docker_net

networks:
  docker_net:
    external: true
```

Where `./ca-api` on the host contains the YAML shown in [Configuration](#configuration).


## Storage layout

For each Certificate Authority a separate folder is created at `{config_path}/{caName}/`:

```
{config_path}/
└── {caName}/                       # one directory per CA (caName == CA Common Name)
    ├── ca_cert.pem                 # CA root certificate (PEM)
    ├── ca_private_key.pem          # CA private key (PEM, PKCS8, RSA-4096)
    ├── serial                      # next client-cert serial (decimal u64, plain text)
    ├── crl_number                  # next CRL number (decimal u64, plain text)
    ├── index.json                  # journal of all issued/revoked certs (see below)
    ├── crl.pem                     # current CRL, regenerated on every revoke
    └── certs/
        └── {email_with_@_replaced_by_underscore}/
            ├── cert.pem            # client certificate (PEM)
            └── private_key.pem     # client private key (PEM, PKCS8, RSA-4096)
```

`index.json` is the source of truth for what was issued and what is revoked:

```json
[
  {
    "cn": "alice@example.com",
    "serial": 1,
    "issued_at": 1714000000,
    "revoked_at": null
  },
  {
    "cn": "bob@example.com",
    "serial": 2,
    "issued_at": 1714000100,
    "revoked_at": 1714050000
  }
]
```

`issued_at` / `revoked_at` are Unix timestamps in seconds. On every revoke, the corresponding record is updated and `crl.pem` is rebuilt from all entries with `revoked_at != null`.

All files are standard PEM / JSON — inspect with `openssl x509 -in ca_cert.pem -text -noout`, `openssl crl -in crl.pem -text -noout`, etc.

PKCS#12 bundles returned by `/api/certificates/v1/downloadCert` use the modern format (AES-256-CBC + HMAC-SHA-256 + PBKDF2-SHA-256). macOS Keychain, Windows ≥ 10 1809, and modern Linux trust stores accept them as-is.


## HTTP API

All routes listen on port `8000` (mapped to `5959` in the compose example above). Swagger UI is exposed at `/swagger/index.html`.

### Certificate Authority

| Method | Route | Input | Purpose |
|--------|-------|-------|---------|
| POST   | `/api/ca/v1/generate`           | body: `caName`, `organization`, `countryCode`, `city` | Create a new CA |
| POST   | `/api/ca/v1/check`              | body: `caName` | Return 200 if the CA exists, 404 otherwise |
| POST   | `/api/ca/v1/import`             | multipart: `caName`, `cert` (PEM file), `privateKey` (PEM file) | Import an existing CA |
| GET    | `/api/ca/v1/list`               | — | List CAs (folder names under `config_path` that contain `ca_cert.pem`) |
| GET    | `/api/ca/v1/downloadCert`       | query: `caName` | Download CA cert as PEM text |
| GET    | `/api/ca/v1/downloadPk`         | query: `caName` | Download CA private key as PEM text |
| GET    | `/api/ca/v1/downloadRevokedPem` | query: `caName` | Download per-CA CRL as PEM text |

### Client certificates (always scoped to a `caName`)

| Method | Route | Input | Purpose |
|--------|-------|-------|---------|
| POST   | `/api/certificates/v1/generate`     | body: `caName`, `email` | Issue a client cert (CN = `email`, RSA-4096, 10y validity) |
| DELETE | `/api/certificates/v1/revoke`       | query: `caName`, `email` | Mark cert revoked and regenerate `crl.pem` |
| GET    | `/api/certificates/v1/downloadCert` | query: `caName`, `email`, `password` | Download PKCS#12 (cert + key + CA chain) |
| GET    | `/api/certificates/v1/list`         | query: `caName` | List issued certs for a CA (`[{cn, revoked}]`) |
| GET    | `/api/revoked/v1/crl`               | query: `caName` | Current CRL as PEM text |
| GET    | `/api/revoked/v1/list`              | query: `caName` | Revoked entries with serials and revocation timestamps |

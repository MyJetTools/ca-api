Docker compose example


```yaml

services:
  ca-api:
    image: myjettools/ca-api:0.1.0
    hostname: ca-api
    container_name: ca-api
    restart: always
    deploy:
      resources:
        limits:
          memory: 64Mb
        reservations:
          memory: 64Mb
    volumes:
      - ./.ca-api:/root/.ca-api
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


.ca-api File example

```yaml
config_path: /root/pki
```


## Storage layout

`ca-api` is a self-contained pure-Rust PKI (no `easy-rsa`/`openssl` CLI). Everything is generated and signed via `rcgen` + `openssl` FFI bindings, and persisted as plain files under `config_path`.

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

`index.json` is the source of truth for what was issued and what is revoked. Entries:

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

`issued_at` / `revoked_at` are Unix timestamps (seconds). When a certificate is revoked, the corresponding record is updated and `crl.pem` is rebuilt from all `revoked_at != null` entries.

The CA file layout is decoupled from any particular tool — the files can be inspected and manipulated with standard `openssl x509 -in ca_cert.pem -text -noout`, `openssl crl -in crl.pem -text -noout`, etc.


## HTTP API

| Method | Route | Purpose |
|--------|-------|---------|
| POST   | `/api/ca/v1/generate`         | Create a new CA (`caName`, `organization`, `countryCode`, `city`) |
| POST   | `/api/ca/v1/check`            | Check whether a CA exists |
| POST   | `/api/ca/v1/import`           | Import an existing CA (cert + private key as multipart files) |
| GET    | `/api/ca/v1/list`             | List CAs (folder names under `config_path`) |
| GET    | `/api/ca/v1/downloadCert`     | Download CA cert PEM (`?caName=...`) |
| GET    | `/api/ca/v1/downloadPk`       | Download CA private key PEM (`?caName=...`) |
| GET    | `/api/ca/v1/downloadRevokedPem` | Download per-CA CRL PEM (`?caName=...`) |
| POST   | `/api/certificates/v1/generate` | Issue client cert (`caName`, `email`) |
| DELETE | `/api/certificates/v1/revoke` | Revoke client cert (`?caName=...&email=...`) |
| GET    | `/api/certificates/v1/downloadCert` | Download PKCS#12 (`?caName=...&email=...&password=...`) |
| GET    | `/api/certificates/v1/list`   | List issued certs for a CA (`?caName=...`) |
| GET    | `/api/revoked/v1/crl`         | Current CRL PEM (`?caName=...`) |
| GET    | `/api/revoked/v1/list`        | Revoked entries with serials and revocation dates (`?caName=...`) |

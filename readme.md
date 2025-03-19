# Dev environment setup

- need docker installed
- need to install sqlx-cli
  - `cargo install sqlx-cli`

## To run dev services

```bash
cd tools
docker-compose -up
```

## db sqlx-cli stuff

```bash
sqlx migrate run
```

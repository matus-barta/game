# Dev environment setup

- need docker installed
- need to install diesel-cli
  - on MacOS: `brew install diesel`

## To run dev services

```bash
cd tools
docker-compose -up
```

## Setup Diesel

**probably ignore... sigh**

```bash
export DIESEL_CONFIG_FILE="./lib_db/db/diesel.toml"
```

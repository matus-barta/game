# Where were we

## Asset server

- [x] connect to postgres
- [x] connect to S3
- [x] load .env
- [x] spawn http api server
- [x] GET /health endpoint
- [ ] GET /chunk/{id}
  - [x] stub
  - [ ] load from DB
- [ ] POST /chunk
  - [ ] decide how chunk id works [thinking here](./docs/chunk.md)
  - [ ] raw dog json to DB table
- [x] GET /model/{id}
  - [x] find in DB get id
  - [x] presign S3 URL
  - [x] return model info (url, id, name)
- [ ] POST /model
  - [x] file upload (client to asses server)
  - [ ] file validation
    - [ ] filename sanitization
    - [x] empty values
    - [x] filetype
  - [x] hash file (sloow) - hash=id
  - [x] upload to S3
  - [x] make new entry in DB

## Client (a.k.a the game) - in Godot (for now)

- [x] load GLTF assets
- [x] spawn downloaded assets
- [x] download chunk
- [ ] finnish game? 😅

## DB

### assets

- [x] chunk table
- [x] placables table
- [x] models table
- [x] world objects table

### user info

- [ ] table with users

## Login server

- [ ] http server
- [ ] auth endpoint
- [ ] how to do _secure_ auth???
- [ ] create user
- [ ] auth user

## Character server

- [ ] load character
- [ ] create character

## World server router (master server)

- [ ] idk

## World server

- [ ] run WS server
- [ ] process players
- [ ] process npc
  - [ ] spawn
  - [ ] move
  - [ ] some logic
- [ ] scripting (this is big one)
- [ ] many more...

## Tools

- [x] setup docker compose with minio, Postgres and pgadmin
- [x] setup configs for pgadmin
- ???

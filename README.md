[![Build Status](https://drone.mjy-home.duckdns.org/api/badges/michael/pantry-manager-api/status.svg?ref=refs/heads/main)](https://drone.mjy-home.duckdns.org/michael/pantry-manager-api)

[Cargo watch](https://github.com/watchexec/cargo-watch) and [sea-orm-cli](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/) are suggested. Watch to run the program while also making live changes. sea-orm-cli to manage the entities and migrations.

Libraries used:
* rocket
* sea-orm

Create migration `sea-orm-cli migrate generate m20220525_1351_create_another`
Cargo watch `cargo watch -x 'run --bin pantry-manager-api'`
Create entities `sea-orm-cli generate entity --database-url "postgres://USERNAME:PASSWORD@IP/HOSTNAME:5432/test" -o entity/src/entities`

To change the database connection you need to change `default.databases.pantry_manager -> url` in Rocket.toml.

This project currently uses [Edaman](https://www.edamam.com/) API for getting product information from the UPC. You will need an account with them and to get some information and place it in `Pantry.toml` or as an environment variable:

* App Id -> `edaman_app_id` OR `-e PANTRY_API_edaman_app_id`
* App Key -> `edaman_app_key` OR `-e PANTRY_API_edaman_app_key`

Added OpenAPI via Okapi. You can see it by going to `http://localhost:8000/swagger/index.html`

## Docker

sudo docker build . -t pantry-manager-api

sudo docker run --name pantry-manager-api -d -p 8000:8000 -e ROCKET_log_level=normal pantry-manager-api

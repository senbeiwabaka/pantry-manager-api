Cargo watch and sea-orm-cli are suggested. Watch to run the program while also making live changes. sea-orm-cli to manage the entities and migrations.

Libraries used:
* rocket
* sea-orm

Create migration `sea-orm-cli migrate generate m20220525_1351_create_another`
Cargo watch `cargo watch -x 'run --bin pantry-manager-api'`
Create entities `sea-orm-cli generate entity --database-url "postgres://USERNAME:PASSWORD@IP/HOSTNAME:5432/test" -o entity/src/entities`

To change the database connection you need to change `default.databases.pantry_manager -> url` in Rocket.toml.

This project currently uses [Edaman](https://www.edamam.com/) API for getting product information from the UPC. You will need an account with them and take that token from the account and place it in either the `Pantry.toml` or as an environment variable `PANTRY_API_edaman_api_key`.

Added OpenAPI via Okapi. You can see it by going to `http://localhost:8000/swagger/index.html`


## Docker

sudo docker build . -t pantry-manager-api

sudo docker run --name pantry-test -d -p 8000:8000 -e ROCKET_log_level=debug -e PANTRY_API_edaman_api_key=7d3b33551msh95e109d0bffdbd9p1b41d7jsn58568e27de5d pantry-manager-api
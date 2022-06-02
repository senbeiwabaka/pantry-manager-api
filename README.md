Cargo watch and sea-orm-cli are suggested. Watch to run the program while also making live changes. sea-orm-cli to manage the entities and migrations.

Libraries used:
* rocket
* sea-orm

Create migration `sea-orm-cli migrate generate m20220525_1351_create_another`
Cargo watch `cargo watch -x 'run --bin pantry-manager-api'`
Create entities `sea-orm-cli generate entity --database-url "postgres://postgres:mysecretpassword@infrastructure-pi-2.home.arpa:5432/test" -o entity/src/entities`

To change the database connection you need to change `default.databases.pantry_manager -> url` in Rocket.toml.
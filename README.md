# MAC0332 - Search service

## Build

To execute the project using Docker, run the following commands:

```
cd search-service

docker compose build

docker compose up
```
## Environment Variables (inside docker compose)

```
DB_HOST: dbms host
DB_PORT: dbms port
DB_USER: dbms user
DB_PASS: dbms password
DB_NAME: database name
ALLOWED_SCHEMAS: database schemas that can be queried
```

## Tests

To execute the tests, make sure the database is correctly populated, then run:

```
cargo test
```

## Sample Database population

1. [PostgreSQL source and instructions](database/sample/postgresql/README.md)

<br>
<br>

# Architecture

## Front-end parser

![Front-end parser](docs/front_end_parser.png)

## Command

![Command Architecture](<docs/command_architecture.png>)

## Visitor

![Vistior Architecture](docs/visitor_achitecture.png)


## Query representation

### Initial representation
![Initial representation](docs/initial_representation.png)

### Intermediary representation
![Intermediary representation](docs/intermediary_representation.png)

### Ultimate representation
![Ultimate representation](docs/ultimate_representation.png)
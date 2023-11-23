## Sample source

[databasestar/sample-database-movies](https://www.databasestar.com/sample-database-movies/)

### Entity–relationship model

[Diagram link](https://www.databasestar.com/wp-content/uploads/2019/11/movies_erd.png)

## Setup.sh

```
USAGE:
    setup.sh -U <database_user> -d <database_name> -p <database_password> [OPTIONS]

REQUIRED PARAMS:
    -U | --user: username for connecting to postgres
    -p | --password: password for connecting to postgres
    -d | --database: database name

OPTIONS:
    -h | --help: print this help message
    -H | --host: database host (default is localhost)
    -P | --port: database port (default is 5432)
```
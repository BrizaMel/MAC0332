
## Before running setup.sh


```sudo -iu postgres psql```

``` 
CREATE USER <user_name>;
ALTER USER <user_name> WITH PASSWORD '<new_password>';
CREATE DATABASE <database_name>;
GRANT ALL PRIVILEGES ON DATABASE <database_name> TO <user_name>;
\c <database_name>
GRANT ALL PRIVILEGES ON SCHEMA PUBLIC to <user_name>;
```

## Sample source


https://www.databasestar.com/sample-database-movies/

### Entity–relationship model

https://www.databasestar.com/wp-content/uploads/2019/11/movies_erd.png

## Setup.sh

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
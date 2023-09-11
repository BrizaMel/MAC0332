#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

DML_DIR=$SCRIPT_DIR"/dml/"
DDL_FILE=$SCRIPT_DIR"/ddl.sql"

usage() {
    cat <<EOF
USAGE:
    setup.sh -U <database_user> -d <database_name> -p <database_password> [OPTIONS]

IMPORTANT: Don't forget to read README.md before running this script

REQUIRED PARAMS:
    -U | --user: username for connecting to postgres
    -p | --password: password for connecting to postgres
    -d | --database: database name

OPTIONS:
    -h | --help: print this help message
    -H | --host: database host (default is localhost)
    -P | --port: database port (default is 5432)
EOF
}

DATABASE_NAME=""
DATABASE_USER=""
DATABASE_PASSWORD=""
DATABASE_HOST="localhost"
DATABASE_PORT="5432"
while [[ $# -gt 0 ]]; do
    case "$1" in
    -h | --help)
        usage
        exit 0
        ;;
    -U | --user)
        DATABASE_USER="$2"
        shift
        shift
        ;;
    -p | --password)
        DATABASE_PASSWORD="$2"
        shift
        shift
        ;;
    -d | --database)
        DATABASE_NAME="$2"
        shift
        shift
        ;;
    -H | --host)
        DATABASE_HOST="$2"
        shift
        shift
        ;;
    -P | --port)
        DATABASE_PORT="$2"
        shift
        shift
        ;;
    -*)
        echo "unknown option '$1'."
        shift
        ;;
    *)
        echo default
        exit 1
        ;;
    esac
done

# Asseret that required parameters were given

if [ -z "$DATABASE_NAME" ]; then
    echo "missing database name"
    usage
    exit 1
fi

if [ -z "$DATABASE_USER" ]; then
    echo "missing database username"
    usage
    exit 1
fi

if [ -z "$DATABASE_PASSWORD" ]; then
    echo "missing database password"
    usage
    exit 1
fi

connection="postgres://$DATABASE_USER:$DATABASE_PASSWORD@$DATABASE_HOST:$DATABASE_PORT/$DATABASE_NAME"


ddl_database(){
    error=$(psql $connection -f $DDL_FILE 2>&1 | grep -i "error")
    if [[ "$error" != "" ]]; then
        echo "$error"
        return 1
    else
        echo "SUCCESS - DDL"
        return 0
    fi
}

dml_database(){
    dml_errors=""
    for filename in $(ls $DML_DIR | grep ".sql") ; do
        error=$(psql $connection -f $DML_DIR'/'$filename 2>&1 | grep -i "error")
        if [[ "$error" != "" ]];then
            dml_errors=$dml_errors$'\n'$error
        fi
    done
    if [[ "$dml_errors" != "" ]]; then
        echo "$error"
        return 1
    else
        echo "SUCCESS - DML"
        return 0
    fi
}


main(){
    ddl_database
    ddl_return=$?
    if [[ "$ddl_return" == 1 ]];then
        echo "ABORTING"
        exit
    fi
    dml_database
    dml_return=$?
    if [[ "$dml_return" == 1 ]];then
        echo "ABORTING"
        exit
    fi 
}

main


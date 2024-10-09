#!/bin/bash

# script to start SurrealDB on FileSystem,
# the DB is saved to folder named `store_db`
# chmod +x start_db.sh
# Connect to the DB using `SurrealList` to visualize

cd ..
mkdir -p "store_db"
surreal start --log warn --user root --pass root --bind 0.0.0.0:9999 rocksdb://store_db/mydatabase.db

## use the below command if you want to query the DB from the command-line

# surreal sql --endpoint http://localhost:9999 --namespace test --database store --auth-level root --username root --password root
rm database.db
diesel setup
sqlite3 database.db ".read sql/total.sql"
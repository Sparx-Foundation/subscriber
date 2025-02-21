# Running Migrator CLI

- Generate a new migration file
    ```sh
    cargo migration generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo migration
    ```
    ```sh
    cargo migration up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo migration up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo migration down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo migration down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo migration fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo migration refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo migration reset
    ```
- Check the status of all migrations
    ```sh
    cargo migration status
    ```

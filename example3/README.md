# :crab: CRUD examples between [Rust](https://www.rust-lang.org/) and [MongoDB](https://www.mongodb.com)

## Requirements

* Rust > 1.70

## Usage

1 - Create a `.env` file in the project root.

2 - Add the following variables:

```
MONGODB_USERNAME=
MONGODB_HOSTNAME=
MONGODB_PASSWORD=
MONGODB_DB_NAME=
MONGODB_CLUSTER=
````

> Note: Fill in according to the data in your database.

3 - Run the test to verify the connection:

```
cargo test tests::database::connection
````

---
[LICENSE](https://github.com/williamcanin/crud_rust_mongodb/blob/main/LICENSE)
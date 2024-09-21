# Shuttle TiDB Demo

This is a simple demo of using TiDB with Shuttle.

- For TiDB Serverless, start [here](https://tidb.cloud). 
- For Shuttle, start [here](https://docs.shuttle.rs/getting-started/installation)


Access [shuttle-tidb-demo](https://shuttle-tidb-demo.shuttleapp.rs/) and see the result:

```json
{"message":"Hello, 5.7.28-TiDB-v7.1.3-serverless"}
```

Or you can create a `Secrets.dev.toml` file with your own TiDB connection string and run the app locally:

```toml
DATABASE_URL = "mysql://your_account:your_password@gateway01.us-west-2.prod.aws.tidbcloud.com:4000/test"
```

```bash
cargo shuttle run
```
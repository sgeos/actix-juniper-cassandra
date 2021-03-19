# Actix-Juniper-Cassandra

Simple Cassandra/GraphQL example with Actix/Rust.

## Usage

### server

```bash
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8080
```

### web client

[http://127.0.0.1:8080/graphiql](http://127.0.0.1:8080/graphiql)

_Query example:_
```graphql
query($id: Int!) {
  apiVersion
  
  ping {
    success
    pong
    date
    time
    datetime
  }
  
  user(id: $id) {
    id
    name
  }
}
```

_Query variables:_
```json
{
  "id": 1
}
```

_Result:_
```json
{
  "data": {
    "apiVersion": "0.1.0",
    "ping": {
      "success": true,
      "pong": "Pong",
      "date": "2021-03-19",
      "time": "16:23:16",
      "datetime": "2021-03-19T16:23:16.598065+00:00"
    },
    "user": {
      "id": 1,
      "name": "Aaron"
    }
  }
}
```


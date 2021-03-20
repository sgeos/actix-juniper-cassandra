# Actix-Juniper-Cassandra

Simple Cassandra/GraphQL example with Actix/Rust.

## Usage

### server

```bash
HOST=127.0.0.1 PORT=8080 cargo run (or ``HOST=127.0.0.1 PORT=8080 cargo watch -x run``)
# Started http server: 127.0.0.1:8080
```

### web client

[http://127.0.0.1:8080/graphiql](http://127.0.0.1:8080/graphiql)

_Query example:_
```graphql
query($id: Int!) {
  ping {
    success
    pong
    date
    time
    datetime
  }
  
  info {
    version
    remoteIp
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
    "ping": {
      "success": true,
      "pong": "Pong",
      "date": "2021-03-20",
      "time": "15:25:25",
      "datetime": "2021-03-20T15:25:25.519969+00:00"
    },
    "info": {
      "version": "0.1.0",
      "remoteIp": "127.0.0.1:49424",
      "date": "2021-03-20",
      "time": "15:25:25",
      "datetime": "2021-03-20T15:25:25.520111+00:00"
    },
    "user": {
      "id": 1,
      "name": "Aaron"
    }
  }
}
```


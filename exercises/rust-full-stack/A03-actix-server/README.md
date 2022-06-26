## branch 05

```
cargo run -p webservice --bin server1

❯ curl http://127.0.0.1:3000/health
"Actix Web Service is running!"

或者:
ce webservice && cargo run --bin server1
```

## branch 06

```sh
cd webservice && cargo run

❯ curl http://127.0.0.1:3000/health
"I'm OK. 0 times"
❯ curl http://127.0.0.1:3000/health
"I'm OK. 1 times"
```

## branch 06-2 post rest-api

```sh
cd webservice && cargo run

❯ curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"teacher_id":1, "name":"First course"}'
"Course added"
```

## branch 06-3 get rest-api

```sh
❯ curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"teacher_id":1, "name":"First course"}'
"Course added"
❯ curl http://127.0.0.1:3000/courses/1
[{"teacher_id":1,"id":1,"name":"First course","time":"2022-03-21T14:30:25.223233906"}]
```

```sh
❯ curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"teacher_id":1, "name":"course 3"}'
"Course added"
❯ curl http://127.0.0.1:3000/courses/1/2
{"teacher_id":1,"id":2,"name":"course 2","time":"2022-03-21T14:41:30.927924580"}
```

### Lint

```
let len = iterator.clone().collect::<Vec<_>>().len();
// should be
let len = iterator.count();
```

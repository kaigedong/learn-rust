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

## Actix-Web 中的错误处理

Actix-Web 把错误转化为 HTTP Response

- Actix-Web 定义了一个通用的错误类型（struct）：actix_web::error::Error, 它实现了了 std::error::Error 这个 trait

- 任何实现了标准库 Error trait 的类型，都可以通过 ? 运算符，转化为 Actix 的 Error 类型

- Actix 的 Error 类型会自动转化为 HTTP Response，返回给客户端

- Actix-Web 里面定义了 ResponseError trait，任何实现该 trait 的错误均可转化为 HTTP Response 消息

- 内置的实现：Actix-Web 对常见错误有内置的实现，例如：

  - Rust 标准 I/O 错误
  - Serde 错误
  - Web 错误，例如 ProtocolError, Utf8Error, ParseError 等等

- 其他错误类型: 内置实现不可用时，需要自定义实现错误到 HTTP Response 的转换

### 创建自定义错误处理器

1. 实现一个自定义错误类型
2. 实现 From trait，用于将其他错误类型转化为该类型
3. 为自定义错误类型实现 ResponseError trait
4. 在 handler 里返回自定义错误类型
5. Actix 会把错误转化为 HTTP 响应

## 在当前目录下，存在两个 bin 时，sqlx offline 的方法：

```sh
cargo sqlx prepare -- --bin actix-teacher-service
```

## 最后一步：部署

(cargo build 不能对wasm进行构建)

wasm-pack build --release

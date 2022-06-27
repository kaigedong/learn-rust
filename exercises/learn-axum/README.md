# Axum 练习

## 01: 基本用法

```shell
# 测试运行服务：
cargo run --example basic
pip install httpie
http "http://127.0.0.1:8080/"
```

```console
HTTP/1.1 200 OK
content-length: 13
content-type: text/html; charset=utf-8
date: Sun, 20 Feb 2022 08:23:14 GMT

Hello, world!
```

```shell
http "http://127.0.0.1:8080/todos"
```

```console
HTTP/1.1 200 OK
content-length: 89
content-type: application/json
date: Sun, 20 Feb 2022 08:23:23 GMT

[
    {
        "completed": false,
        "id": 1,
        "title": "Todo 1"
    },
    {
        "completed": false,
        "id": 2,
        "title": "Todo 2"
    }
]
```

```shell
# 类型解析已经被Axum优雅的处理了:
http post "http://127.0.0.1:8080/todos"
```

```console
HTTP/1.1 400 Bad Request
content-length: 54
date: Sun, 20 Feb 2022 08:26:26 GMT

Expected request with `Content-Type: application/json`
```

```shell
http post "http://127.0.0.1:8080/todos" id=1
```

```console
HTTP/1.1 400 Bad Request
content-length: 83
date: Sun, 20 Feb 2022 08:27:56 GMT

Failed to parse the request body as JSON: missing field `title` at line 1 column 11
```

```shell
http post "http://127.0.0.1:8080/todos" title=hello
```

```console
HTTP/1.1 201 Created
content-length: 0
date: Sun, 20 Feb 2022 08:28:42 GMT
```

## 02: 细节实现

```shell
# 使用jsonwebtoken实现了验证用户登录信息后返回验证token的功能
http post "http://127.0.0.1:8080/login" email=1@qq.com password=abc
```

```console
HTTP/1.1 200 OK
content-length: 131
content-type: application/json
date: Sun, 20 Feb 2022 10:23:10 GMT

{
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IkthaWdlIERvbmcifQ.0Oas_zm23Nz3K0IYPIlVFFZTT-dKjnJZzDZS1P7Ajh8"
}
```

```shell
# 将返回的验证token上传并验证

http post "http://127.0.0.1:8080/todos" title=hello "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IkthaWdlIERvbmcifQ.0Oas_zm23Nz3K0IYPIlVFFZTT-dKjnJZzDZS1P7Ajh8"
```

```console
HTTP/1.1 401 Unauthorized
content-length: 12
content-type: text/plain; charset=utf-8
date: Sun, 20 Feb 2022 10:50:25 GMT

Unauthorized
```

- 解决 `Unauthorized` 的问题

通过打印错误，提示：`Error:: Error(MissingRequiredClaim("exp"))`，因此`Claims`需要有一个`exp`字段

```rust
struct Claims {
    id: usize,
    exp: usize, //Required. Expiration time (as UTC timestamp)
    name: String,
}

// 其中，exp为Unix timestamp, 如1655002720，为绝对时间，而不是相对时间。
```

## 03. 线程间共享变量

```shell
# 获取认证Token
http post "http://127.0.0.1:8080/login" email=1@qq.com password=abc
```

```console
HTTP/1.1 200 OK
content-length: 153
content-type: application/json
date: Mon, 21 Feb 2022 03:18:11 GMT

{
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwiZXhwIjoxNjQ2NjIzMDkxLCJuYW1lIjoiS2FpZ2UgRG9uZyJ9.6FBAEKt543hwZWlWU3awPebZJOBLAB1JH1YMB9oHFMs"
}
```

```shell
# 存储用户的Todo
http post "http://127.0.0.1:8080/login" email=1@qq.com password=abc
```

```console
HTTP/1.1 200 OK
content-length: 153
content-type: application/json
date: Mon, 21 Feb 2022 03:18:11 GMT

{
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwiZXhwIjoxNjQ2NjIzMDkxLCJuYW1lIjoiS2FpZ2UgRG9uZyJ9.6FBAEKt543hwZWlWU3awPebZJOBLAB1JH1YMB9oHFMs"
}
```

```shell
# 获取用户的Todo
http "http://127.0.0.1:8080/todos" "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwiZXhwIjoxNjU1MDAyNzIwLCJuYW1lIjoiS2FpZ2UgRG9uZyJ9.QIaZobhf9fEjJrVKqk1Lz23Bb4Y03rjobojd3wZo1dc"

```

```console
HTTP/1.1 200 OK
content-length: 56
content-type: application/json
date: Mon, 21 Feb 2022 03:19:34 GMT

[
    {
        "completed": false,
        "id": 1,
        "title": "hello",
        "user_id": 1
    }
]
```

## 04. 打包前端静态文件

当进行分发时，有将静态文件打包到二进制中分发给用户的需求。可以使用 rust-embed 来处理该需求

```shell
# 创建一个react-app
npx create-react-app my-app
cd my-app
npm run build
# npm start

# 测试：
cargo run --example basic

http "http://127.0.0.1:8080"
```

```console
HTTP/1.1 200 OK
content-length: 644
content-type: text/html
date: Mon, 21 Feb 2022 05:07:34 GMT

<!doctype html><html lang="en"><head><meta charset="utf-8"/><link rel="icon" href="/favicon.ico"/><meta name="viewport" content="width=device-width,initial-scale=1"/><meta name="theme-color" content="#000000"/><meta name="description" content="Web site created using create-react-app"/><link rel="apple-touch-icon" href="/logo192.png"/><link rel="manifest" href="/manifest.json"/><title>React App</title><script defer="defer" src="/static/js/main.0278e899.js"></script><link href="/static/css/main.073c9b0a.css" rel="stylesheet"></head><body><noscript>You need to enable JavaScript to run this app.</noscript><div id="root"></div></body></html>
```

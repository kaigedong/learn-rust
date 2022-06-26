## 4.2 测试：

```sh
cargo run -p httpserver
```

```sh
# Index
http://localhost:3000/
# Health page
http://localhost:3000/health
# api
http://localhost:3000/api/shipping/orders
# 404
http://localhost:3000/api/shipping
http://localhost:3000/api/123
```

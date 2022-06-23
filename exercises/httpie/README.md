# 实现一个类似 httpie 的命令行小工具

## 使用的库：

- clap 进行命令行解析

- reqwest 的异步接口来进行 http 请求

- colored 进行命令终端显示

- anyhow 错误处理

- jsonxf 格式化 JSON 响应

- mime 处理 mime 类型

- toki 做异步处理

```
cargo build --quiet && target/debug/httpie post httpbin.org/post a=1 b=2
```

- [x] 使用 syntect 继续完善 HTTPie 进行 HTTP body 的语法高亮

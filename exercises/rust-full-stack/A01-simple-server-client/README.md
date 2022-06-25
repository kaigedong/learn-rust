# Rust 全栈

## 最简单的 HTTP 服务

### 运行

```sh
cargo run --bin tcpserver

cargo run --bin tcpclient
```

### 要点：

#### 1. `.write`/`.read` 和 `.write_all`/`.read_exact`

#### 2. 为什么 write_all 不需要`&mut[u8]参数` 而是`&[u8]`

`write_all`的函数签名如下，可以看到，它接收了一个`&[u8]`的参数，说明不需要改变参数的内容。

```rust
#[stable(feature = "rust1", since = "1.0.0")]
fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
    while !buf.is_empty() {
        match self.write(buf) {
            Ok(0) => {
                return Err(error::const_io_error!(
                    ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ));
            }
            Ok(n) => buf = &buf[n..],
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
```

但有些奇怪的是，它是 `mut buf`。假如有如下签名：

```rust
fn write_all(&mut self, buf: &[u8]) -> Result<()> {}
```

我们想实现后面的逻辑`buf = &buf[n..],`是不可能的，因此要在函数题内这样处理：

```rust
fn write_all(&mut self, buf: &[u8]) -> Result<()> {
    let mut buf = buf;
    ...
}
```

因此，mut buf 相当于将第二个参数处理成了这样的逻辑

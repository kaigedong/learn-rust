- webassembly 环境配置[参考](https://rustwasm.github.io/docs/book/game-of-life/setup.html)：

- 根据模板创建项目：

```bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

- 其中，一个重要的部分是，rust 中调用前端的函数：

```
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// 如果Rust中的函数想要提供给前端调用，就加上这个
#[wasm_bindgen]
pub fn greet() {
    // 调用前端的alert函数
    alert("Hello, wasm-game-of-life!");
}
```

- 编译

```bash
wasm-pack build
# 生成文件位置： pkg/wasm_game_of_life_bg.wasm
```

- 生成前端

```console
❯cd wasm-game-of-life && npm init wasm-app www
Need to install the following packages:
  create-wasm-app@0.1.0
Ok to proceed? (y) y
🦀 Rust + 🕸 Wasm = ❤
```

- 初始化之后，我们需要在 www/package.json 中引入 pkg

```json
"dependencies": {
    "wasm-game-of-life": "file:../pkg"
},
"devDependencies": {
    "hello-wasm-pack": "^0.1.0",
}
```

- 接着，需要安装依赖

```bash
cd www && npm install
```

- www/index.js 中有wasm调用的示例：

```JavaScript
import * as wasm from "hello-wasm-pack";

wasm.greet();
```

变更为：

```bash
import * as wasm from "wasm-game-of-life";

wasm.greet();
```

- 运行：

```console
npm run start
❯ npm run start

> create-wasm-app@0.1.0 start
> webpack-dev-server

ℹ ｢wds｣: Project is running at http://localhost:8080/
```



<div align="center">

  <h1><code>wasm-pack-template</code></h1>

<strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

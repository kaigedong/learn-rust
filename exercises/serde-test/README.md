- Rust 的序列化解决方案不是一个第三方库，而是以 serde 为中心的一系列第三方库。
  包括 serde_json, serde_yaml

内容包括：

1. serde 简介，什么是 Serialization, Deserialization

- Serde: 是 rust 语言用来序列化和反序列化数据的解决方案。特点：
  - 1. 体现了定义与实现分离的设计思想
  - 2. serde 基于 rust 的 trait 来实现自己的序列化
  - 3. 具有较好的性能
- Serialization: 是将内存中的数据结构或对象转换成二进制序列的过程，可以用于网络传输或保存在本地
- Deserialization: 是将序列化过程中所生成的二进制序列转换成数据结构或对象的过程，实际上就是一个相反过程

2. Serialize 与 Serializer: serde_json, serde_yaml

- Serialize 就是定义 && Serializer 就是实现
- Serde 提供一个宏 serde_derive 自动的为结构体类型和枚举类型生成 Serialize 和 Deserialize 的实现,
  注意，这个是定义的实现，而不是实现的实现
  - `#[derive(Serialize)]` 这个标签，**就给数据结构实现了这个定义**
- 但是 Serde 没有实现 Serializer Deserializer
  - 一般情况下，我们用第三方库来实现 Serializer，Deserializer

Serializer:

- 来看一下 serde 的 trait `Serialize`
- 定义一个 Person 类，来实现一下，来看一下 `Serializer`

3. Serde, Json, Yaml 实战

4. 常用场景：修改 Json 的实战

- [ ] 添加 Serializer/Deserializer 的实现

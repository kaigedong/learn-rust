## 什么是 gRPC？

gRPC 是一个现代的开源的高性能远程过程调用(RPC)框架，可以在任何环境下运行。RPC 代表远程过程调用(Remote Procedure Call)，开头的 g 代表通用(General Purpose)，对某些人来说，它代表谷歌。

假设你熟悉常见的 REST api，它们通过 JSON 对象进行通信。在 gRPC 中，我们使用 Protocol Buffers 来序列化数据，而不是 JSON。

## Protocol Buffers

Protocol Buffers 是谷歌定制的用于语言无关、平台无关、可扩展的，序列化结构性数据的一种协议。

在 gRPC 中，传输的(序列化的)数据是二进制的。这意味着它比 JSON 或 XML 更快，因为它占用更少的空间，而更少的空间带来更小的带宽。

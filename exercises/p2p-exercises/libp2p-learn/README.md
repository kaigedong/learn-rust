# 例子 1

## P2p 网络

P2P(Peer-to-Peer)是一种网络技术，可以在网络中不同的计算机上共享各种计算资源，如 CPU、网络带宽和存储等。P2P 技术应用最广泛的是在网络中共享文件以及区块链网络，它们不依赖中央服务器或中介来连接多个客户端，用户的计算机即是客户端也是服务器。

由于 P2P 网络是一种分布式系统，不会像中央服务器一样存在单点故障，因此容错性极强。

下面让我们来看一看 P2P 网络的核心概念：

### 传输协议

在 P2P 网络底层一般使用 TCP/UDP 传输层协议。由于 P2P 节点应用的多样性，在 TCP/UDP 传输层协议之上，会使用多种应用层协议，如：HTTP，gRPC 及自定义协议等。为了有效利用资源，P2P 网络会在一个连接上监听、解析多种协议，即多路复用技术：多个逻辑子流可以在同一个底层(TCP)连接上共存。可以查看 yamux 库了解更多细节。

### 节点标识

P2P 网络中的节点需要一个唯一的标识，以便其他节点能够找到它们。P2P 网络中的节点使用公钥和私钥对(非对称公钥加密)与其他节点建立安全通信。在 P2P 网络中节点标识被称为 PeerId，它是通过对节点公钥进行加密哈希得到的。

### 安全规则

密钥对和节点身份标识使节点之间能够体建立安全的、经过身份验证的通信通道。但这只是安全的一个方面，节点还需要根据业务逻辑实现授权框架，该框架建立一些规则：哪些节点可以执行哪种类型的操作等。

### 节点路由

P2P 网络中的一个节点首先需要找到其他节点来进行通信。这是通过维护一个节点路由表来实现的。但是在 P2P 网络中，有成千上万个节点在动态变化(即节点的加入和离开)，单个节点很难为网络中的所有节点维护一个完整、准确的路由表。所以节点路由表通常会由一系列路由节点维护。

### 消息

P2P 网络中的节点可以向特定节点发送消息，也可以广播消息。使用发布/订阅模式，节点订阅感兴趣 Topic，所有订阅该 Topic 的节点都能接收和发送消息。这种技术也通常用于将消息的内容传输到整个网络。

### 流多路复用

在 P2P 网络中，允许多个独立的“逻辑”流共享一个公共的 P2P 传输层。流多路复用有助于优化节点之间建立网络连接的开销。多路复用在后端服务开发中很常见，客户端可以与服务器建立底层网络连接，然后在底层网络连接上复用不同的流。

## libp2p

自己编写 P2P 应用程序的网络层是一项庞大的工程，我们将使用底层 p2p 网络库—libp2p，在其上构建 p2p 应用程序会更加容易。libp2p 是一个模块化的系统，支持三种编程语言：Rust、Go、JS。许多流行的项目中都使用 libp2p 做为 P2P 网络底层，如 IPFS、Filecoin、Polkadot 和 Substrate。

libp2p 将 P2P 网络基本概念分解成了不同的模块，可以在不同的应用场景中组合使用。

我们先通过 Ping 这个简单的程序来熟悉一下 libp2p 的组件及如何使用 libp2p 开发点对点网络。

```console
❯ cargo run --bin ping --quiet
节点ID: 12D3KooWN4PV5vJba1VFvCwfctvbuV8WtnRA6DsxFDhJ3N8MrrAE
本地监听地址: /ip4/127.0.0.1/tcp/44469
本地监听地址: /ip4/192.168.1.34/tcp/44469
本地监听地址: /ip4/192.168.1.135/tcp/44469
```

可以看到已经打印出 PeerId 和监听地址。

```console
❯ cargo run --bin ping --quiet /ip4/127.0.0.1/tcp/41517
节点ID: 12D3KooWJ7124KZVAoGPLh4XvvyamsTE1ktPauNkdD3gporALm5W
链接远程节点: /ip4/127.0.0.1/tcp/41517
本地监听地址: /ip4/127.0.0.1/tcp/33465
本地监听地址: /ip4/192.168.1.34/tcp/33465
本地监听地址: /ip4/192.168.1.135/tcp/33465
Event { peer: PeerId("12D3KooWLG963dpXV2H4AJQmLUfZDc2xcSwDM3aLaNAjeFndU6kc"), result: Ok(Pong) }
Event { peer: PeerId("12D3KooWLG963dpXV2H4AJQmLUfZDc2xcSwDM3aLaNAjeFndU6kc"), result: Ok(Ping { rtt: 806.596µs }) }
```

我们可以看到链接到刚刚的节点/ip4/127.0.0.1/tcp/58645 成功，也收到了发送过来的 ping/pong 的消息。

系列文章：
https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247484736&idx=1&sn=005ccb89105809b68790600086da5b7a&chksm=cfc2a823f8b5213589c57d003cf971a4d64e60146b0089819c228a0221a118482767ffdbdc9b&scene=178&cur_album_id=2361592668113420289#rd

---

# 例子 2

这个聊天程序在 PING 程序的基础上加入了节点自动发现机制，消息发送采用了发布/订阅模式，节点间的网络传输做了加密处理。

因此需要使用以下协议：

    floodsub协议：是libp2p几种pub/sub方案之一，适用于小型网络中消息的广播。

    mDNS协议：本地节点发现协议。

    noise协议：传输层安全加密协议。

在 libp2p 中，floodsub 和 mDNS 属于网络行为，我们创建自己的网络行为将他们组合在一起。

打开两个终端，分别运行：

```console
cargo run --bin chat
```

第一个节点输出：

```console
master:libp2p-learn Justin$ cargo run --bin chat
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/chat`
节点ID: 12D3KooWFt1xhyL5ZfJAR2SxUAQLEeb4ryTtQ5T2VtZsNGRKmB5J
本地监听地址: /ip4/127.0.0.1/tcp/52318
在网络中加入节点: 12D3KooWLDojjNzDHYUZCpqV4AL4XTMGScvHHAe8vsJrpCpYv2pT
在网络中加入节点: 12D3KooWLDojjNzDHYUZCpqV4AL4XTMGScvHHAe8vsJrpCpYv2pT
```

第二个节点输出：

```console
master:libp2p-learn Justin$ cargo run --bin chat
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/chat`
节点ID: 12D3KooWLDojjNzDHYUZCpqV4AL4XTMGScvHHAe8vsJrpCpYv2pT
本地监听地址: /ip4/127.0.0.1/tcp/52320
在网络中加入节点: 12D3KooWFt1xhyL5ZfJAR2SxUAQLEeb4ryTtQ5T2VtZsNGRKmB5J
在网络中加入节点: 12D3KooWFt1xhyL5ZfJAR2SxUAQLEeb4ryTtQ5T2VtZsNGRKmB5J
```

两个节点自动发现了对方，并加入 P2P 网络。

我们在第二个节点中发送“hello”，第一个节点打印出：

```console
收到消息: '"hello"' 来自 PeerId("12D3KooWLDojjNzDHYUZCpqV4AL4XTMGScvHHAe8vsJrpCpYv2pT")
```

然后第一个节点回复“world”，第二个节点打印出：

```console
收到消息: '"world"' 来自 PeerId("12D3KooWFt1xhyL5ZfJAR2SxUAQLEeb4ryTtQ5T2VtZsNGRKmB5J")
```

## gRPC

RPC: Remote Procedure Call 的简称，远程过程调用

使用场景：A 应用 调用远程服务 B 的方法

需要交互约定：

1. 调用的语义，也可以理解为接口规范。(比如 RESTful)
2. 网络传输协议 (比如 HTTP)
3. 数据序列化反序列化规范(比如 JSON、protobuf)。

gRPC: 是由 google 开发的一个`高性能`的`通用`的开源`RPC框架`，主要面向移动应用开发且基于 HTTP/2 协议标准而设计，同时支持大多数流行的编程语言。

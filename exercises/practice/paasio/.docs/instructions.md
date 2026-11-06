# Instructions

# 指示

Report network IO statistics.

报告网络 IO 统计信息。

You are writing a [PaaS][paas], and you need a way to bill customers based on network and filesystem usage.

您正在编写一个 [PaaS][paas]，并且您需要一种根据网络和文件系统使用情况向客户计费的方法。

Create a wrapper for network connections and files that can report IO statistics.
The wrapper must report:

为可以报告 IO 统计信息的网络连接和文件创建包装器。
包装者必须报告：

- The total number of bytes read/written.
- The total number of read/write operations.

- 读取/写入的总字节数。
- 读/写操作的总数。

[paas]: https://en.wikipedia.org/wiki/Platform_as_a_service


[paas]：https://en.wikipedia.org/wiki/Platform_as_a_service
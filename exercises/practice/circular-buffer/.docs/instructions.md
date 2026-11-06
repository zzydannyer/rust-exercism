# Instructions

# 指示

A circular buffer, cyclic buffer or ring buffer is a data structure that uses a single, fixed-size buffer as if it were connected end-to-end.

循环缓冲区、循环缓冲区或环形缓冲区是一种使用单个固定大小缓冲区的数据结构，就好像它是端到端连接的一样。

A circular buffer first starts empty and of some predefined length.
For example, this is a 7-element buffer:

循环缓冲区首先是空的并且具有某个预定义的长度。
例如，这是一个 7 元素缓冲区：

```text
[ ][ ][ ][ ][ ][ ][ ]
```

Assume that a 1 is written into the middle of the buffer (exact starting location does not matter in a circular buffer):

假设将 1 写入缓冲区的中间（在循环缓冲区中，确切的起始位置并不重要）：

```text
[ ][ ][ ][1][ ][ ][ ]
```

Then assume that two more elements are added — 2 & 3 — which get appended after the 1:

然后假设添加了另外两个元素——2 和 3——它们被附加在 1 之后：

```text
[ ][ ][ ][1][2][3][ ]
```

If two elements are then removed from the buffer, the oldest values inside the buffer are removed.
The two elements removed, in this case, are 1 & 2, leaving the buffer with just a 3:

如果随后从缓冲区中删除两个元素，则缓冲区内最旧的值将被删除。
在本例中，删除的两个元素是 1 和 2，缓冲区中只剩下 3：

```text
[ ][ ][ ][ ][ ][3][ ]
```

If the buffer has 7 elements then it is completely full:

如果缓冲区有 7 个元素，则它已完全满：

```text
[5][6][7][8][9][3][4]
```

When the buffer is full an error will be raised, alerting the client that further writes are blocked until a slot becomes free.

当缓冲区已满时，将引发错误，警告客户端进一步的写入将被阻止，直到插槽空闲为止。

When the buffer is full, the client can opt to overwrite the oldest data with a forced write.
In this case, two more elements — A & B — are added and they overwrite the 3 & 4:

当缓冲区已满时，客户端可以选择通过强制写入来覆盖最旧的数据。
在本例中，添加了另外两个元素 — A 和 B — 并且它们覆盖了 3 和 4：

```text
[5][6][7][8][9][A][B]
```

3 & 4 have been replaced by A & B making 5 now the oldest data in the buffer.
Finally, if two elements are removed then what would be returned is 5 & 6 yielding the buffer:

3 和 4 已被 A 和 B 取代，使得 5 现在成为缓冲区中最旧的数据。
最后，如果删除两个元素，则返回的是 5 和 6，产生缓冲区：

```text
[ ][ ][7][8][9][A][B]
```

Because there is space available, if the client again uses overwrite to store C & D then the space where 5 & 6 were stored previously will be used not the location of 7 & 8.
7 is still the oldest element and the buffer is once again full.

由于还有可用空间，如果客户端再次使用覆盖来存储 C 和 D，则将使用之前存储 5 和 6 的空间，而不是 7 和 8 的位置。
7 仍然是最旧的元素，缓冲区再次满了。

```text
[C][D][7][8][9][A][B]
```

# Instructions

# 指示

Your task is to look for patterns in the long sequence of digits in the encrypted signal.

您的任务是在加密信号的长数字序列中寻找模式。

The technique you're going to use here is called the largest series product.

您将在此处使用的技术称为最大系列产品。

Let's define a few terms, first.

首先让我们定义几个术语。

- **input**: the sequence of digits that you need to analyze
- **series**: a sequence of adjacent digits (those that are next to each other) that is contained within the input
- **span**: how many digits long each series is
- **product**: what you get when you multiply numbers together

- **输入**：您需要分析的数字序列
- **系列**：输入中包含的一系列相邻数字（彼此相邻的数字）
- **跨度**：每个系列的长度是多少位
- **乘积**：将数字相乘得到的结果

Let's work through an example, with the input `"63915"`.

让我们看一个示例，输入为 `"63915"`。

- To form a series, take adjacent digits in the original input.
- If you are working with a span of `3`, there will be three possible series:
  - `"639"`
  - `"391"`
  - `"915"`
- Then we need to calculate the product of each series:
  - The product of the series `"639"` is 162 (`6 × 3 × 9 = 162`)
  - The product of the series `"391"` is 27 (`3 × 9 × 1 = 27`)
  - The product of the series `"915"` is 45 (`9 × 1 × 5 = 45`)
- 162 is bigger than both 27 and 45, so the largest series product of `"63915"` is from the series `"639"`.
  So the answer is **162**.


- 要形成一个系列，请取原始输入中的相邻数字。
- 如果您使用 `3` 的跨度，则将存在三个可能的系列：
  - ZXQ保护1QXZ
  - ZXQ保护2QXZ
  - `"915"`
- 然后我们需要计算每个级数的乘积：
  - `"639"`系列产品为162 (`6 × 3 × 9 = 162`)
  - `"391"`系列产品为27 (`3 × 9 × 1 = 27`)
  - `"915"`系列产品为45 (`9 × 1 × 5 = 45`)
- 162比27和45都大，所以`"63915"`最大的系列产品来自`"639"`系列。
  所以答案是**162**。
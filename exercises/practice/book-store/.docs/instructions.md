# Instructions

# 指示

To try and encourage more sales of different books from a popular 5 book series, a bookshop has decided to offer discounts on multiple book purchases.

为了尝试鼓励热门 5 本书系列中不同书籍的销量增加，一家书店决定对购买多本书提供折扣。

One copy of any of the five books costs $8.

五本书中任何一本的售价为 8 美元。

If, however, you buy two different books, you get a 5% discount on those two books.

但是，如果您购买两本不同的书，则这两本书可享受 5% 的折扣。

If you buy 3 different books, you get a 10% discount.

如果您购买 3 本书，您将获得 10% 的折扣。

If you buy 4 different books, you get a 20% discount.

如果您购买 4 本书，您将获得 20% 的折扣。

If you buy all 5, you get a 25% discount.

如果您购买全部 5 件，则可享受 25% 的折扣。

Note that if you buy four books, of which 3 are different titles, you get a 10% discount on the 3 that form part of a set, but the fourth book still costs $8.

请注意，如果您购买四本书，其中 3 本书是不同的书名，则您可以对构成套装一部分的 3 本书享受 10% 的折扣，但第四本书仍需花费 8 美元。

Your mission is to write code to calculate the price of any conceivable shopping basket (containing only books of the same series), giving as big a discount as possible.

你的任务是编写代码来计算任何可能的购物篮（仅包含同系列的书籍）的价格，并给出尽可能大的折扣。

For example, how much does this basket of books cost?

例如，这篮子书多少钱？

- 2 copies of the first book
- 2 copies of the second book
- 2 copies of the third book
- 1 copy of the fourth book
- 1 copy of the fifth book

- 第一本书的两本
- 第二本书2本
- 第三本书2本
- 一本第四本书
- 第五本书一本

One way of grouping these 8 books is:

将这 8 本书分组的一种方法是：

- 1 group of 5 (1st, 2nd,3rd, 4th, 5th)
- 1 group of 3 (1st, 2nd, 3rd)

- 1组，每组5人（第一、第二、第三、第四、第五）
- 1组3人（第一、第二、第三）

This would give a total of:

这总共将给出：

- 5 books at a 25% discount
- 3 books at a 10% discount

- 5 本书，25% 折扣
- 3 本书可享受 10% 的折扣

Resulting in:

导致：

- 5 × (100% - 25%) × $8 = 5 × $6.00 = $30.00, plus
- 3 × (100% - 10%) × $8 = 3 × $7.20 = $21.60

- 5 × (100% - 25%) × 8 美元 = 5 × 6.00 美元 = 30.00 美元，加上
- 3 × (100% - 10%) × 8 美元 = 3 × 7.20 美元 = 21.60 美元

Which equals $51.60.

相当于 51.60 美元。

However, a different way to group these 8 books is:

然而，对这 8 本书进行分组的另一种方式是：

- 1 group of 4 books (1st, 2nd, 3rd, 4th)
- 1 group of 4 books (1st, 2nd, 3rd, 5th)

- 1 组 4 本书（第 1、第 2、第 3、第 4）
- 1 组 4 本书（第 1、第 2、第 3、第 5）

This would give a total of:

这总共将给出：

- 4 books at a 20% discount
- 4 books at a 20% discount

- 4 本书，20% 折扣
- 4 本书，20% 折扣

Resulting in:

导致：

- 4 × (100% - 20%) × $8 = 4 × $6.40 = $25.60, plus
- 4 × (100% - 20%) × $8 = 4 × $6.40 = $25.60

- 4 × (100% - 20%) × 8 美元 = 4 × 6.40 美元 = 25.60 美元，加上
- 4 × (100% - 20%) × 8 美元 = 4 × 6.40 美元 = 25.60 美元

Which equals $51.20.

相当于 51.20 美元。

And $51.20 is the price with the biggest discount.


51.20 美元是折扣最大的价格。
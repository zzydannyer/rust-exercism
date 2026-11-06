# Instructions

# 指示

Make a chain of dominoes.

制作多米诺骨牌链。

Compute a way to order a given set of domino stones so that they form a correct domino chain.
In the chain, the dots on one half of a stone must match the dots on the neighboring half of an adjacent stone.
Additionally, the dots on the halves of the stones without neighbors (the first and last stone) must match each other.

计算一种方法来对一组给定的多米诺骨牌进行排序，以便它们形成正确的多米诺骨牌链。
在链中，一块石头的一半上的点必须与相邻石头的相邻一半上的点相匹配。
此外，没有邻居的一半棋子（第一颗和最后一颗棋子）上的点必须彼此匹配。

For example given the stones `[2|1]`, `[2|3]` and `[1|3]` you should compute something
like `[1|2] [2|3] [3|1]` or `[3|2] [2|1] [1|3]` or `[1|3] [3|2] [2|1]` etc, where the first and last numbers are the same.

For stones `[1|2]`, `[4|1]` and `[2|3]` the resulting chain is not valid: `[4|1] [1|2] [2|3]`'s first and last numbers are not the same.
4 != 3

Some test cases may use duplicate stones in a chain solution, assume that multiple Domino sets are being used.


一些测试用例可能会在链解决方案中使用重复的石头，假设正在使用多个 Domino 集。
# Instructions

# 指示

Your task is to write the code that calculates the energy points that get awarded to players when they complete a level.

您的任务是编写代码来计算玩家完成关卡时获得的能量点。

The points awarded depend on two things:

授予的积分取决于两件事：

- The level (a number) that the player completed.
- The base value of each magical item collected by the player during that level.

- 玩家完成的级别（数字）。
- 玩家在该关卡中收集的每个魔法物品的基础价值。

The energy points are awarded according to the following rules:

能量点根据以下规则授予：

1. For each magical item, take the base value and find all the multiples of that value that are less than the level number.
2. Combine the sets of numbers.
3. Remove any duplicates.
4. Calculate the sum of all the numbers that are left.

1. 对于每个魔法物品，取基值并找到该值的所有小于等级数的倍数。
2. 合并各组数字。
3. 删除所有重复项。
4. 计算剩下的所有数字的总和。

Let's look at an example:

让我们看一个例子：

**The player completed level 20 and found two magical items with base values of 3 and 5.**

**玩家完成了20级并发现了两个基础值分别为3和5的魔法物品。**

To calculate the energy points earned by the player, we need to find all the unique multiples of these base values that are less than level 20.

为了计算玩家获得的能量点，我们需要找到这些基础值小于 20 级的所有唯一倍数。

- Multiples of 3 less than 20: `{3, 6, 9, 12, 15, 18}`
- Multiples of 5 less than 20: `{5, 10, 15}`
- Combine the sets and remove duplicates: `{3, 5, 6, 9, 10, 12, 15, 18}`
- Sum the unique multiples: `3 + 5 + 6 + 9 + 10 + 12 + 15 + 18 = 78`
- Therefore, the player earns **78** energy points for completing level 20 and finding the two magical items with base values of 3 and 5.


- 20以下的3的倍数：`{3, 6, 9, 12, 15, 18}`
- 20以下5的倍数：`{5, 10, 15}`
- 组合集合并删除重复项：`{3, 5, 6, 9, 10, 12, 15, 18}`
- 对唯一倍数求和：`3 + 5 + 6 + 9 + 10 + 12 + 15 + 18 = 78`
- 因此，玩家完成 20 级并找到基础值为 3 和 5 的两个魔法物品即可获得 **78** 能量点。
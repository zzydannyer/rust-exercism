# Instructions

# 指示

Given a person's allergy score, determine whether or not they're allergic to a given item, and their full list of allergies.

根据一个人的过敏评分，确定他们是否对特定物品过敏，以及他们的过敏的完整列表。

An allergy test produces a single numeric score which contains the information about all the allergies the person has (that they were tested for).

过敏测试会产生一个单一的数字分数，其中包含有关该人（他们接受测试的）所有过敏的信息。

The list of items (and their value) that were tested are:

已测试的项目（及其价值）列表为：

- eggs (1)
- peanuts (2)
- shellfish (4)
- strawberries (8)
- tomatoes (16)
- chocolate (32)
- pollen (64)
- cats (128)

- 鸡蛋 (1)
- 花生 (2)
- 贝类 (4)
- 草莓 (8)
- 西红柿 (16)
- 巧克力 (32)
- 花粉 (64)
- 猫 (128)

So if Tom is allergic to peanuts and chocolate, he gets a score of 34.

因此，如果汤姆对花生和巧克力过敏，他的得分为 34。

Now, given just that score of 34, your program should be able to say:

现在，给出 34 分，您的程序应该能够说：

- Whether Tom is allergic to any one of those allergens listed above.
- All the allergens Tom is allergic to.

- 汤姆是否对上面列出的任何一种过敏原过敏。
- 汤姆对所有过敏原过敏。

Note: a given score may include allergens **not** listed above (i.e. allergens that score 256, 512, 1024, etc.).
Your program should ignore those components of the score.
For example, if the allergy score is 257, your program should only report the eggs (1) allergy.


注意：给定分数可能包括上面未列出的过敏原（即得分 256、512、1024 等的过敏原）。
您的程序应该忽略分数的这些组成部分。
例如，如果过敏评分为 257，则您的程序应仅报告鸡蛋 (1) 过敏。
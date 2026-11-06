# Instructions

# 指示

Your task is to determine which items to take so that the total value of her selection is maximized, taking into account the knapsack's carrying capacity.

你的任务是考虑背包的承载能力，确定要拿走哪些物品，以使她选择的总价值最大化。

Items will be represented as a list of items.
Each item will have a weight and value.
All values given will be strictly positive.
Lhakpa can take only one of each item.

项目将表示为项目列表。
每件物品都有重量和价值。
所有给出的值都将严格为正值。
拉巴每样物品只能拿走一件。

For example:

例如：

```text
Items: [
  { "weight": 5, "value": 10 },
  { "weight": 4, "value": 40 },
  { "weight": 6, "value": 30 },
  { "weight": 4, "value": 50 }
]

Knapsack Maximum Weight: 10
```

For the above, the first item has weight 5 and value 10, the second item has weight 4 and value 40, and so on.
In this example, Lhakpa should take the second and fourth item to maximize her value, which, in this case, is 90.
She cannot get more than 90 as her knapsack has a weight limit of 10.


对于上述内容，第一个项目的权重为 5，值 10，第二个项目的权重为 4，值 40，依此类推。
在此示例中，Lhakpa 应该采取第二项和第四项来最大化她的价值，在本例中为 90。
她不能超过 90，因为她的背包重量限制为 10。
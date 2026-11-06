# Introduction

# 介绍

Your friend Eliud inherited a farm from her grandma Tigist.
Her granny was an inventor and had a tendency to build things in an overly complicated manner.
The chicken coop has a digital display showing an encoded number representing the positions of all eggs that could be picked up.

你的朋友埃鲁德从她的祖母提吉斯特那里继承了一座农场。
她的奶奶是一位发明家，倾向于以过于复杂的方式建造东西。
鸡舍有一个数字显示屏，显示一个编码数字，代表可以拾取的所有鸡蛋的位置。

Eliud is asking you to write a program that shows the actual number of eggs in the coop.

Eliud 要求你编写一个程序来显示鸡舍中鸡蛋的实际数量。

The position information encoding is calculated as follows:

位置信息编码计算如下：

1. Scan the potential egg-laying spots and mark down a `1` for an existing egg or a `0` for an empty spot.
2. Convert the number from binary to decimal.
3. Show the result on the display.

1. 扫描潜在的产卵点，并为现有卵标记 `1`，或为空点标记 `0`。
2. 将数字从二进制转换为十进制。
3. 在显示屏上显示结果。

## Example 1

## 示例 1

![Seven individual nest boxes arranged in a row whose first, third, fourth and seventh nests each have a single egg.](https://assets.exercism.org/images/exercises/eliuds-eggs/example-1-coop.svg)

!ZXQ保护0QXZ

```text
 _ _ _ _ _ _ _
|E| |E|E| | |E|
```

### Resulting Binary

### 生成的二进制文件

![1011001](https://assets.exercism.org/images/exercises/eliuds-eggs/example-1-binary.svg)

!ZXQ保护0QXZ

```text
 _ _ _ _ _ _ _
|1|0|1|1|0|0|1|
```

### Decimal number on the display

### 显示的十进制数

89

### Actual eggs in the coop

### 鸡舍里的实际鸡蛋

4

## Example 2

## 示例2

![Seven individual nest boxes arranged in a row where only the fourth nest has an egg.](https://assets.exercism.org/images/exercises/eliuds-eggs/example-2-coop.svg)

!ZXQ保护0QXZ

```text
 _ _ _ _ _ _ _
| | | |E| | | |
```

### Resulting Binary

### 生成的二进制文件

![0001000](https://assets.exercism.org/images/exercises/eliuds-eggs/example-2-binary.svg)

!ZXQ保护0QXZ

```text
 _ _ _ _ _ _ _
|0|0|0|1|0|0|0|
```

### Decimal number on the display

### 显示的十进制数

8

### Actual eggs in the coop

### 鸡舍里的实际鸡蛋

1

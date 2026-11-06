# Instructions

# 指示

Your task is to, given a diagram, determine which plants each child in the kindergarten class is responsible for.

你的任务是根据给定的图表确定幼儿园班级中每个孩子负责哪些植物。

There are 12 children in the class:

班上有12名小朋友：

- Alice, Bob, Charlie, David, Eve, Fred, Ginny, Harriet, Ileana, Joseph, Kincaid, and Larry.

- 爱丽丝、鲍勃、查理、大卫、伊芙、弗雷德、金妮、哈里特、伊利亚娜、约瑟夫、金凯德和拉里。

Four different types of seeds are planted:

种植四种不同类型的种子：

| Plant  | Diagram encoding |
| ------ | ---------------- |
| Grass  | G                |
| Clover | C                |
| Radish | R                |
| Violet | V                |

Each child gets four cups, two on each row:

每个孩子得到四个杯子，每行两个：

```text
[window][window][window]
........................ # each dot represents a cup
........................
```

Their teacher assigns cups to the children alphabetically by their names, which means that Alice comes first and Larry comes last.

他们的老师按照孩子们名字的字母顺序为他们分配杯子，这意味着爱丽丝排在第一位，拉里排在最后。

Here is an example diagram representing Alice's plants:

这是代表爱丽丝植物的示例图：

```text
[window][window][window]
VR......................
RG......................
```

In the first row, nearest the windows, she has a violet and a radish.
In the second row she has a radish and some grass.

在第一排最靠近窗户的地方，她有一棵紫罗兰和一个萝卜。
第二排有一个萝卜和一些草。

Your program will be given the plants from left-to-right starting with the row nearest the windows.
From this, it should be able to determine which plants belong to each student.

您的程序将从左到右从最靠近窗户的行开始获得植物。
由此，它应该能够确定哪些植物属于每个学生。

For example, if it's told that the garden looks like so:

例如，如果被告知花园看起来像这样：

```text
[window][window][window]
VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV
```

Then if asked for Alice's plants, it should provide:

然后，如果要求提供爱丽丝的植物，它应该提供：

- Violets, radishes, violets, radishes

- 紫罗兰，萝卜，紫罗兰，萝卜

While asking for Bob's plants would yield:

询问鲍勃的植物会得到：

- Clover, grass, clover, clover


- 三叶草，草，三叶草，三叶草
# Instructions

# 指示

Optical Character Recognition or OCR is software that converts images of text into machine-readable text.
Given a grid of characters representing some digits, convert the grid to a string of digits.
If the grid has multiple rows of cells, the rows should be separated in the output with a `","`.

光学字符识别（OCR）是将文本图像转换为机器可读文本的软件。
给定代表某些数字的字符网格，将网格转换为数字字符串。
如果网格有多行单元格，则应在输出中使用 `","` 分隔各行。

- The grid is made of one of more lines of cells.
- Each line of the grid is made of one or more cells.
- Each cell is three columns wide and four rows high (3x4) and represents one digit.
- Digits are drawn using pipes (`"|"`), underscores (`"_"`), and spaces (`" "`).

## Edge cases

## 边缘情况

- If the input is not a valid size, your program should indicate there is an error.
- If the input is the correct size, but a cell is not recognizable, your program should output a `"?"` for that character.

- 如果输入的大小无效，您的程序应指示存在错误。
- 如果输入的大小正确，但单元格无法识别，则您的程序应为该字符输出 `"?"`。

## Examples

## 示例

The following input (without the comments) is converted to `"1234567890"`.

以下输入（不带注释）将转换为 `"1234567890"`。

```text
      _  _     _  _  _  _  _  _  #
    | _| _||_||_ |_   ||_||_|| | # Decimal numbers.
    ||_  _|  | _||_|  ||_| _||_| #
                                 # The fourth line is always blank,
```

The following input is converted to `"123,456,789"`.

以下输入将转换为 `"123,456,789"`。

<!-- prettier-ignore-start -->

<!-- 更漂亮的忽略开始 -->

```text
    _  _ 
  | _| _|
  ||_  _|
         
    _  _ 
|_||_ |_ 
  | _||_|
         
 _  _  _ 
  ||_||_|
  ||_| _|
         
```

<!-- prettier-ignore-end -->


<!-- 更漂亮的忽略结束 -->
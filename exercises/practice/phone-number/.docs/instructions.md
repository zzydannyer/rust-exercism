# Instructions

# 指示

Clean up phone numbers so that they can be sent SMS messages.

清理电话号码，以便可以向其发送短信。

The **North American Numbering Plan (NANP)** is a telephone numbering system used by many countries in North America like the United States, Canada or Bermuda.
All NANP-countries share the same international country code: `1`.

**北美编号计划 (NANP)** 是北美许多国家/地区（例如美国、加拿大或百慕大）使用的电话编号系统。
所有 NANP 国家/地区共享相同的国际国家/地区代码：`1`。

NANP numbers are ten-digit numbers consisting of a three-digit Numbering Plan Area code, commonly known as _area code_, followed by a seven-digit local number.
The first three digits of the local number represent the _exchange code_, followed by the unique four-digit number which is the _subscriber number_.

NANP 号码是十位数的号码，由三位数的编号计划区号（通常称为_区号_）组成，后跟一个七位数的本地号码。
本地号码的前三位数字代表_交换代码_，后跟唯一的四位数字，即_用户号码_。

The format is usually represented as

格式通常表示为

```text
NXX NXX-XXXX
```

where `N` is any digit from 2 through 9 and `X` is any digit from 0 through 9.

其中 `N` 是 2 到 9 之间的任意数字，`X` 是 0 到 9 之间的任意数字。

Sometimes they also have the country code (represented as `1` or `+1`) prefixed.

有时，它们还带有国家/地区代码（表示为 `1` 或 `+1`）前缀。

Your task is to clean up differently formatted telephone numbers by removing punctuation and the country code if present.

您的任务是通过删除标点符号和国家/地区代码（如果存在）来清理不同格式的电话号码。

For example, the inputs

例如，输入

- `+1 (613)-995-0253`
- `613-995-0253`
- `1 613 995 0253`
- `613.995.0253`

should all produce the output

应该全部产生输出

`6139950253`

**Note:** As this exercise only deals with telephone numbers used in NANP-countries, only 1 is considered a valid country code.


**注意：** 由于此练习仅涉及 NANP 国家/地区使用的电话号码，因此只有 1 被视为有效的国家/地区代码。
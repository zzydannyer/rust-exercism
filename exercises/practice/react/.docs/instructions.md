# Instructions

# 指示

Implement a basic reactive system.

实施基本的反应系统。

Reactive programming is a programming paradigm that focuses on how values are computed in terms of each other to allow a change to one value to automatically propagate to other values, like in a spreadsheet.

响应式编程是一种编程范例，重点关注如何相互计算值，以允许对一个值的更改自动传播到其他值，就像在电子表格中一样。

Implement a basic reactive system with cells with settable values ("input" cells) and cells with values computed in terms of other cells ("compute" cells).
Implement updates so that when an input value is changed, values propagate to reach a new stable system state.

使用具有可设置值的单元（“输入”单元）和具有根据其他单元计算的值的单元（“计算”单元）来实现基本反应系统。
实施更新，以便当输入值更改时，值会传播以达到新的稳定系统状态。

In addition, compute cells should allow for registering change notification callbacks.
Call a cell’s callbacks when the cell’s value in a new stable state has changed from the previous stable state.


此外，计算单元应该允许注册更改通知回调。
当新稳定状态下的单元格值较之前的稳定状态发生变化时，调用单元格的回调。
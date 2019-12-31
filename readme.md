+ 式微，式微！胡不归？
+ 不行了，不行了，你怎么还在写程序？

zhscript 是一种中文语法构成的脚本语言。

### hello, world

```
显示循环【11】【次】先
	循环【‘次’】先
		分叉‘次’先
			1：h。
			2：e。
			3、4、10：l。
			5、8：o。
			7：w。
			9：r。
			11：d。
			“, ”
		了
	了换行
了。
```

### 运行

```bash
$ cargo run src2/hw.zs
h
ee
lll
llll
ooooo
, , , , , , 
wwwwwww
oooooooo
rrrrrrrrr
llllllllll
ddddddddddd
```

或

```bash
$ cargo run -- -zhscript-src-is-code "显示循环【11】【次】先循环【‘次’】先分叉‘次’先1：h。2：e。3、4、10：l。5、8：o。7：w。9：r。11：d。“, ”了了换行了"
```

亦同效。

[这里](src2/)还有[《式微》](src2/lose.zs)
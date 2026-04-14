# 教材修订笔记（学习中顺手记）

记录格式建议：

- 章节：`rust-course/src/...`
- 问题：我在哪句话/哪个例子卡住了？
- 建议：补一句解释/换一个例子/增加一个反例/增加一个练习

---

- 章节：`rust-course/src/first-try/installation.md`
  - 问题：Windows 用户即使 `rustc`/`cargo` 已能识别，第一次 `cargo run` 仍可能因 `link.exe` 缺失失败，这个坑对新手很常见。
  - 建议：在“检查安装是否成功”附近补一句“最好再实际运行一次 `cargo new hello && cargo run` 验证链接器可用”，避免误以为环境已经完全就绪。

- 章节：`rust-course/src/basic/variable.md`
  - 问题：新手容易把“遮蔽”和“可变变量”混淆，只知道都能‘改值’，但不知道一个是新绑定、一个是原绑定修改。
  - 建议：在遮蔽部分增加一组 `mut` 与 shadowing 的对照例子，特别强调“shadowing 可以改变类型，mut 不可以”。

- 章节：`rust-course/src/basic/base-type/index.md`
  - 问题：新手容易把 `.expect("...")` 里的字符串误解成类型约束，从而忽略真正报错点是 `parse()` 缺少目标类型信息。
  - 建议：在 `parse` 示例后补一句“`expect` 的字符串只是 panic 提示，不参与类型推导”，避免读者把问题理解歪。

- 章节：`rust-course/src/basic/base-type/index.md`
  - 问题：新手很容易把“有无分号”和“语句/表达式/代码块返回值”之间的关系说反，尤其是在函数返回值场景里。
  - 建议：增加一个并排对照例子，例如 `fn f1() -> i32 { 3 }` 与 `fn f2() -> i32 { 3; }`，帮助建立稳定直觉。

- 章节：`rust-course/src/basic/base-type/numbers.md`
  - 问题：新手容易把 `b'A'` 和 `'A'` 看成只是两种写法，忽略前者是 `u8` 字节字面量、后者是 `char` 字符字面量。
  - 建议：在字面量表格后立刻补一个对照例子：`let x: u8 = b'A'; let y: char = 'A';`，帮助读者建立“字节 vs 字符”区别。

- 章节：`rust-course/src/basic/base-type/numbers.md`
  - 问题：`usize`/`isize` 很容易被初学者只当成“会跟平台变的整数”，但不清楚为什么下标、长度、容量常用 `usize`。
  - 建议：在介绍 `usize` 时补一句“它与平台地址宽度一致，因此适合表示索引和内存相关尺寸”，把“是什么”连到“为什么这样用”。

- 章节：`rust-course/src/basic/base-type/numbers.md`
  - 问题：浮点数部分讲了 `0.1 + 0.2 == 0.3` 的陷阱，但新手未必能立刻抽象出实战原则。
  - 建议：在示例后直接补一句“工程上通常比较 `abs(a-b) < epsilon`，而不是直接 `a == b`”，让读者拿走一个可执行规则。

- 章节：`rust-course/src/basic/compound-type/string-slice.md`
  - 问题：新手很容易把 `String` 和 `&str` 都笼统地当成“字符串”，但不清楚两者最关键的区别其实是“是否拥有所有权”。
  - 建议：在开头报错示例附近补一张极简对照表：`String = 拥有所有权、可增长`，`&str = 借用的字符串切片`，并明确写出“字符串字面量的类型是 `&str`”。

- 章节：`rust-course/src/basic/compound-type/string-slice.md`
  - 问题：教材讲了中文切片 panic，但新手仍容易把 `&s[..5]` 误解成“前 5 个字符”，没有彻底建立“按字节切片”的直觉。
  - 建议：在字符串切片部分增加一句显眼总结：“`[a..b]` 对字符串来说是字节区间，不是字符区间”，并配一个 ASCII 合法 / 中文越界 panic 的并排对照例子。

- 章节：`rust-course/src/basic/compound-type/string-slice.md`
  - 问题：`replace`、`replace_range`、`pop`、`remove` 这些 API 都放在字符串操作一节里，但初学者很容易混淆“是否修改原串”和“返回值类型”。
  - 建议：在“操作字符串”小节末尾加一个汇总表，至少列出：`push/push_str/clear/replace_range/pop/remove` 修改原串，`replace/replacen` 返回新串；并单独标注 `pop -> Option<char>`、`remove -> char`。

- 章节：`rust-course/src/basic/compound-type/string-slice.md`
  - 问题：`+` 拼接和 `format!` 都能得到新字符串，但教材若不特别点明，初学者很容易忽略两者在“是否拿走原变量所有权”上的差异。
  - 建议：在“连接”小节增加一个对照例子：`s1 + &s2` 后 `s1` 失效、`s2` 仍可用；`format!("{} {}", s1, s2)` 后两者仍可继续使用。

- 章节：`rust-course/src/basic/compound-type/string-slice.md`
  - 问题：`chars()` 和 `bytes()` 都放在“操作 UTF-8 字符串”里，但新手常常记不住各自产出的元素类型。
  - 建议：在该小节补一个极简对照：`chars() -> char`，`bytes() -> u8`，并用 `"中国"` 与 `"abc"` 各给一段最短示例。

- 章节：`rust-course/src/basic/compound-type/tuple.md`
  - 问题：新手容易把元组索引写错位，明明想取第二个元素，却写成 `.2`；教材虽提到“从 0 开始”，但冲击力还不够。
  - 建议：在 `.0/.1/.2` 示例后补一个一句话提醒，例如“第二个元素是 `.1`，不是 `.2`”，最好再加一个最短判断题帮助读者当场自测。

- 章节：`rust-course/src/basic/compound-type/tuple.md`
  - 问题：`let (_, y, _) = tup` 这种写法里，新手容易把 `_` 误解成某种匿名变量，而不是“显式忽略该位置”。
  - 建议：在解构部分补一句“`_` 不会绑定变量名，它表示忽略该位置的值”，并给一个“只取中间值”的最短例子。

- 章节：`rust-course/src/basic/compound-type/tuple.md`
  - 问题：教材提到元组可返回多个值，但对“含 `String` 的元组解构后为何不能再整体使用”点得不够透，新手容易只记语法，不联想到 move。
  - 建议：在函数返回元组后增加一个所有权对照例子：`(i32, f64)` 可继续整体使用，`(String, i32)` 解构后不能再 `println!("{:?}", tup)`，把元组语法和所有权规则明确连起来。

- 章节：`rust-course/src/basic/compound-type/struct.md`
  - 问题：新手很容易误以为“只修改某个字段，所以只需要那个字段可变”，但 Rust 实际要求整个结构体绑定是 `mut`。
  - 建议：在字段修改示例后补一句显眼提醒：“Rust 不支持只把结构体某个字段单独标记为可变；要修改字段，必须让整个实例绑定可变”。

- 章节：`rust-course/src/basic/compound-type/struct.md`
  - 问题：结构体更新语法 `..user1` 容易让初学者误解成“把整个旧结构体复制一份”，从而忽略 `String` 等非 `Copy` 字段其实会 move。
  - 建议：在 `..user1` 示例后增加一个极简表格或并排说明：`bool/u64 -> Copy`，`String -> move`，并明确写一句“旧实例可能还能访问部分 `Copy` 字段，但通常不能再整体使用”。

- 章节：`rust-course/src/basic/compound-type/struct.md`
  - 问题：元组结构体与单元结构体都在后半段出现，但新手很容易把 `struct Point(i32, i32, i32);` 错认成单元结构体，只因为它看起来“没有字段名”。
  - 建议：在两种特殊结构体之间增加一个一眼能看懂的对照：`struct Point(i32, i32); // 有字段但无字段名`，`struct Marker; // 没有任何字段`，帮助读者建立稳定区分。

- 章节：`rust-course/src/basic/compound-type/enum.md`
  - 问题：新手很容易把“变量的类型”和“变体携带的数据形状”混淆，例如把 `Message::Write(String::from("hi"))` 误认为 `String` 类型，而不是 `Message`。
  - 建议：在 `Message` 例子后补一个显眼提醒：`Message::Quit`、`Message::Write(...)`、`Message::Move { ... }` 这些值的整体类型都仍然是 `Message`，只是变体内部携带的数据不同；最好配一个最短判断题让读者立刻自测。

- 章节：`rust-course/src/basic/compound-type/enum.md`
  - 问题：教材展示了“结构体 + 枚举”与“直接在枚举变体中携带数据”两种扑克牌写法，但初学者容易只看到“后者代码更短”，没有抓住真正优势是“分支与该分支对应的数据形状被绑定进同一个类型”。
  - 建议：在扑克牌例子后补一句总结：枚举不仅能表示“多选一”，还能把“每一种情况该带什么数据”直接编码进类型，避免出现字段组合合法性不清的设计。

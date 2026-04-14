# 课程路线（以 Rust 语言圣经为纲）

教材目录锚点：`rust-course/src/SUMMARY.md`

原则：先把“所有权/借用/生命周期”学得很牢，再扩展到并发、async 与工程化。

## Phase 0：启动（1-2 课）

- `rust-course/src/first-try/installation.md`
- `rust-course/src/first-try/cargo.md`
- `rust-course/src/first-try/hello-world.md`

## Phase 1：基础核心（建议 10-18 课）

- 变量/类型/表达式/函数：`rust-course/src/basic/variable.md` + `rust-course/src/basic/base-type/index.md`
- 所有权与借用（重点反复回收）：`rust-course/src/basic/ownership/index.md`
- 复合类型 + 模式匹配：`rust-course/src/basic/compound-type/intro.md` + `rust-course/src/basic/match-pattern/intro.md`
- 方法、泛型、trait：`rust-course/src/basic/method.md` + `rust-course/src/basic/trait/intro.md`
- 集合、生命周期、错误处理：`rust-course/src/basic/collections/intro.md` + `rust-course/src/basic/lifetime.md` + `rust-course/src/basic/result-error/intro.md`
- 包与模块、文档、格式化输出：`rust-course/src/basic/crate-module/intro.md` + `rust-course/src/basic/comment.md` + `rust-course/src/basic/formatted-output.md`

## Phase 2：入门实战（5-7 课）

- 文件搜索工具：`rust-course/src/basic-practice/intro.md`

## Phase 3：进阶（按需选修/主修，12-20 课）

- 高级生命周期、闭包/迭代器、类型系统：`rust-course/src/advance/intro.md`
- 智能指针与内部可变性：`rust-course/src/advance/smart-pointer/intro.md`
- 并发与 async：`rust-course/src/advance/concurrency-with-threads/intro.md` + `rust-course/src/advance/async/intro.md`
- unsafe、宏：`rust-course/src/advance/unsafe/intro.md` + `rust-course/src/advance/macro.md`

## Phase 4：进阶实战（项目式）

- Web 服务器：`rust-course/src/advance-practice1/intro.md`
- 简易 Redis（Tokio）：`rust-course/src/advance-practice/intro.md`

## Phase 5：工程化与长期战斗力

- 测试：`rust-course/src/test/intro.md`
- Cargo：`rust-course/src/cargo/intro.md`
- 编译错误专题：`rust-course/src/compiler/intro.md`

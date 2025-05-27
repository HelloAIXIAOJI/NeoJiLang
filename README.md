# NeoJiLang (NJIL) 解释器

用JSON编程！

NeoJiLang (NJIL) 是一个基于JSON语法的解释型语言，使用Rust开发。

NeoJiLang 的前身是 JiLang，但因屎太多终止开发。即使JiLang已经理论图灵完备并且我投入了大量时间。

这是独立于 JiLang 的新项目，JiLang 的JIL或JL文件与 NeoJiLang 的NJIL不互通。

JiLang存储库（https://github.com/HelloAIXIAOJI/JiLang）不会归档。如果您有兴趣，可尝试清理JiLang的屎。我（HelloAIXIAOJI）会第一时间通过并合并你的PR。并将您刻在JiLang的解释器与NeoJiLang的解释器

## 支持的文件格式

NeoJiLang支持两种文件格式：

1. **NJIL (.njil)** - 原始格式，使用复杂的JSON结构表示程序
2. **NJIS (.njis)** - 简化格式，使用JSON数组直接表示语句序列

### NJIL与NJIS的区别

#### NJIL格式 (.njil)

NJIL是一种结构化的格式，主要特点是：

- 使用嵌套的JSON对象组织程序结构
- 包含`import`部分用于导入其他文件或内置模块
- 包含`program`部分，其中定义了函数，如`main`函数
- 每个函数包含`body`数组，其中是实际的执行语句
- 更加正式和结构化，适合复杂程序

NJIL示例：
```json
{
  "import": [
    "!io"
  ],
  "program": {
    "main": {
      "body": [
        {"print": "你好，NJIL！"},
        {"var.set": {"message": "这是一个测试消息"}},
        {"print": {"var": "message"}},
        {"return": {"string.concat": [
          "解释器测试成功！",
          {"var": "message"}
        ]}}
      ]
    }
  }
}
```

#### NJIS格式 (.njis)

NJIS是NJIL的简化版本，主要特点是：

- 直接使用JSON数组表示语句序列，省略了复杂的嵌套结构
- 没有显式的函数定义，整个数组被视为主程序
- 更加简洁和直观，适合简单脚本和原型开发
- 与NJIL共享相同的语句处理逻辑，功能完全一致

NJIS示例：
```json
[
  {"print": "你好，NJIS！"},
  {"var.set": {"message": "这是NJIS格式的测试消息"}},
  {"print": {"var": "message"}},
  {"return": {"string.concat": [
    "NJIS解释器测试成功！",
    {"var": "message"}
  ]}}
]
```

#### 选择指南

- **使用NJIL**：如果您需要开发复杂的程序，需要导入模块，定义多个函数
- **使用NJIS**：如果您需要快速编写简单脚本，或者只关注主程序逻辑

两种格式可以同时使用，根据不同场景选择最适合的格式。

## 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/HelloAIXIAOJI/NeoJiLang.git
cd NeoJiLang

# 编译项目
cargo build --release
```

### 使用示例

创建一个NJIS脚本文件 `example.njis`：

```json
[
  {"print": "你好，NJIL！"},
  {"var.set": {"message": "这是一个测试消息"}},
  {"print": {"var": "message"}},
  {"return": {"string.concat": [
    "解释器测试成功！",
    {"var": "message"}
  ]}}
]
```

运行脚本：

```bash
cargo run -- example.njis
```

或者使用NJIL格式：

```bash
cargo run -- example.njil
```

## 语法参考

NJIL程序由一系列语句组成，每个语句是一个JSON对象，键表示指令，值表示参数。

### 核心语句

- `print`: 打印值到控制台
- `var`: 获取变量值
- `var.set`: 设置变量值
- `return`: 返回值并结束执行
- `string.concat`/`txtlink`: 连接字符串

### 添加新的语句类型

1. 在`statements`目录下创建新的处理器文件
2. 实现`StatementHandler`特质
3. 在`statements/mod.rs`中注册新的处理器

## 版本历史

查看[CHANGELOG.md](CHANGELOG.md)了解详细的版本更新历史。

## 许可证

MIT
# NeoJiLang (NJIL) 解释器

用JSON编程！

NeoJiLang (NJIL) 是一个基于JSON语法的解释型语言，使用Rust开发。

NeoJiLang 的前身是 JiLang，但因屎太多终止开发。即使JiLang已经理论图灵完备并且我投入了大量时间。

这是独立于 JiLang 的新项目，JiLang 的JIL或JL文件与 NeoJiLang 的NJIL不互通。

JiLang存储库（https://github.com/HelloAIXIAOJI/JiLang）不会归档。如果您有兴趣，可尝试清理JiLang的屎。我（HelloAIXIAOJI）会第一时间通过并合并你的PR。并将您刻在JiLang的解释器与NeoJiLang的解释器

## 支持的文件格式

NeoJiLang支持三种文件格式：

1. **NJIL (.njil)** - 原始格式，使用复杂的JSON结构表示程序
2. **NJIS (.njis)** - 简化格式，使用JSON数组直接表示语句序列
3. **NJIM (.njim)** - 模块格式，用于定义可重用的模块

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

#### NJIM格式 (.njim)

NJIM是NeoJiLang的模块格式，用于定义可重用的代码模块：

- 使用JSON对象定义模块的结构和内容
- 包含`module`、`namespace`等元数据字段
- 通过`exports`字段定义模块导出的内容
- 支持导出常量、函数和类型定义
- 作为独立文件存在，可以被其他NJIL程序导入使用
- **注意**：NJIM文件不能直接执行，只能被导入

NJIM示例：
```json
{
  "module": "math",
  "namespace": "math",
  "description": "数学函数模块",
  "author": "NeoJiLang Team",
  "version": "1.0.0",
  "exports": {
    "constants": {
      "PI": 3.14159265359,
      "E": 2.71828182846
    },
    "functions": {
      "add": {
        "body": [
          {
            "return": {
              "add": {
                "a": {"var": "$1"},
                "b": {"var": "$2"}
              }
            }
          }
        ]
      },
      "square": {
        "body": [
          {
            "return": {
              "multiply": {
                "a": {"var": "$1"},
                "b": {"var": "$1"}
              }
            }
          }
        ]
      }
    }
  }
}
```

#### 选择指南

- **使用NJIL**：如果您需要开发复杂的程序，需要导入模块，定义多个函数
- **使用NJIS**：如果您需要快速编写简单脚本，或者只关注主程序逻辑
- **使用NJIM**：如果您需要创建可重用的代码模块，被其他程序导入使用

NJIL和NJIS可以直接执行，NJIM只能被导入使用。

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

## 内置模块

NeoJiLang 提供了多个内置模块，可以通过 `import` 语句导入：

### IO 模块 (!io)

提供输入输出功能：
- `io.read` - 读取用户输入
- `io.read_line` - 读取一行用户输入
- `io.file_read` - 读取文件内容
- `io.file_write` - 写入文件内容

### 日期时间模块 (!datetime)

提供日期和时间处理功能：
- `datetime.date` - 获取当前日期，支持自定义格式
- `datetime.time` - 获取当前时间，支持自定义格式

### Shell 模块 (!shell)

提供终端控制功能：
- `shell.color` - 设置文本颜色，支持前景色和背景色
  ```json
  {"shell.color": {"text": "这是红色文本", "fg": "red"}}
  {"shell.color": {"text": "白色文本, 蓝色背景", "fg": "white", "bg": "blue"}}
  ```
- `shell.style` - 设置文本样式
  ```json
  {"shell.style": {"text": "加粗文本", "bold": true}}
  {"shell.style": {"text": "下划线文本", "underline": true}}
  ```
- `shell.style_color` - 同时设置文本颜色和样式（简化写法）
  ```json
  {"shell.style_color": {"text": "红色加粗文本", "fg": "red", "bold": true}}
  {"shell.style_color": {"text": "蓝底白字加粗下划线", "fg": "white", "bg": "blue", "bold": true, "underline": true}}
  ```
  别名：`shell.cstyle`, `shell.color_style`
- `shell.clear_line` - 清除当前行
  ```json
  {"shell.clear_line": true}
  ```
- `shell.write` - 写入文本（不换行）
  ```json
  {"shell.write": "不换行文本"}
  ```
- `shell.write_line` - 写入文本并换行
  ```json
  {"shell.write_line": "带换行的文本"}
  ```
- `shell.overwrite` - 清除当前行并写入新文本
  ```json
  {"shell.overwrite": "覆盖当前行的文本"}
  ```

支持的颜色：`black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`

支持的样式：`bold`（加粗）, `underline`（下划线）, `blink`（闪烁）

### 常量系统

NeoJiLang支持定义和使用常量，常量是一旦定义就不能修改的值：

- `const.set` - 定义单个常量
  ```json
  {"const.set": {"name": "PI", "value": 3.14159}}
  {"const.set": {"name": "APP_NAME", "value": "我的NeoJiLang应用"}}
  ```

- `const.set.m` - 批量定义多个常量（简洁格式）
  ```json
  {"const.set.m": {
    "MAX_USERS": 100,
    "APP_VERSION": "1.0.0",
    "COLORS": ["红", "绿", "蓝"],
    "CONFIG": {
      "timeout": 5000,
      "max_retry": 3
    }
  }}
  ```
  别名：`const.m`

- `const` - 获取常量值
  ```json
  {"print": {"const": "APP_NAME"}}
  {"var.set": {"name": "timeout", "value": {"const": "CONFIG.timeout"}}}
  ```

- `has_constant` - 检查常量是否存在
  ```json
  {"if": {
    "condition": {"has_constant": "CONFIG"},
    "then": [
      {"print": "CONFIG常量已定义"}
    ],
    "else": [
      {"print": "CONFIG常量未定义"}
    ]
  }}
  ```
  别名：`const.has`, `const.exists`

- 字符串中的常量引用：使用 `${const:名称}` 格式
  ```json
  {"print": "应用名称: ${const:APP_NAME}, 版本: ${const:APP_VERSION}"}
  {"print": "配置的超时时间: ${const:CONFIG.timeout}毫秒"}
  ```

特点：
- 常量一旦定义不可修改，尝试重新定义会抛出错误
- 常量值可以是任何有效的JSON值（数字、字符串、布尔值、数组、对象等）
- 支持嵌套访问，如 `CONFIG.timeout` 或 `COLORS[0]`
- 常量在创建新解释器实例时会被保留，便于跨函数使用
- 适合存储配置信息、数学常数和其他不变的值

示例 - 使用常量进行圆面积计算：
```json
[
  {"const.set": {"name": "PI", "value": 3.14159}},
  {"var.set": {"name": "radius", "value": 5}},
  {"var.set": {"name": "area", "value": {"math.multiply": [
    {"math.multiply": [{"const": "PI"}, {"var": "radius"}]},
    {"var": "radius"}
  ]}}},
  {"print": "半径为${var:radius}的圆面积是: ${var:area}"}
]
```

示例 - 创建进度条：
```json
[
  {"var.set.m": {"progress": 0, "total": 100}},
  {"loop.while": {
    "condition": {"compare": {"left": {"var": "progress"}, "operator": "<", "right": {"var": "total"}}},
    "body": [
      {"shell.clear_line": true},
      {"shell.write": "进度: "},
      {"var.set.m": {"progress": {"add": [{"var": "progress"}, 5]}}},
      {"var.set.m": {"bar": ""}},
      {"loop.for": {
        "count": {"divide": [{"var": "progress"}, 5]},
        "body": {"var.set.m": {"bar": {"string.concat": [{"var": "bar"}, "█"]}}}
      }},
      {"shell.color": {"text": {"var": "bar"}, "fg": "cyan"}},
      {"shell.write": " "},
      {"shell.color": {"text": {"string.concat": [{"var": "progress"}, "%"]}, "fg": "yellow", "bold": true}},
      {"sleep": 100}
    ]
  }},
  {"shell.write_line": ""},
  {"shell.color": {"text": "进度完成！", "fg": "green", "bold": true}}
]
```

## 变量设置

## 模块系统 (NJIM)

NeoJiLang 0.2.0引入了模块系统，通过NJIM文件格式支持代码的模块化和复用。

### 创建模块

创建一个NJIM模块文件：

```json
{
  "module": "math",                 // 模块名称（必填）
  "namespace": "math",              // 命名空间（可选，默认等于模块名）
  "description": "数学函数模块",     // 模块描述（可选）
  "author": "NeoJiLang Team",       // 作者信息（可选）
  "version": "1.0.0",               // 版本号（可选）
  "exports": {                      // 导出内容（必填）
    "constants": {                  // 导出的常量（可选）
      "PI": 3.14159265359,
      "E": 2.71828182846
    },
    "functions": {                  // 导出的函数（可选）
      "add": {
        "body": [
          {
            "return": {             // 所有模块函数必须有返回值
              "add": {
                "a": {"var": "$1"},
                "b": {"var": "$2"}
              }
            }
          }
        ]
      }
    },
    "types": {                      // 导出的类型定义（可选）
      "Point": {
        "x": "number",
        "y": "number"
      }
    }
  },
  "imports": [                      // 该模块依赖的其他模块（可选）
    "other_module.njim"
  ]
}
```

### 使用模块

在NJIL文件中导入和使用模块：

```json
{
  "import": [
    "path/to/math.njim"    // 导入NJIM模块
  ],
  "program": {
    "main": {
      "body": [
        // 访问模块常量
        {"println": {"content": "PI值: ${const:math.PI}"}},
        
        // 调用模块函数
        {"var.set": {
          "name": "result",
          "value": {
            "function.call": {
              "name": "math.add",   // 使用"模块名.函数名"格式调用
              "args": [5, 10]
            }
          }
        }},
        
        {"println": {"content": "5 + 10 = ${var:result}"}}
      ]
    }
  }
}
```

### 模块路径

模块系统支持以下导入路径格式：

- 相对路径: `"../modules/math.njim"`
- 绝对路径: `"/modules/math.njim"`
- 自动添加扩展名: `"modules/math"` (会自动尝试寻找 `modules/math.njim`)

### 模块功能限制

- NJIM模块不能直接执行，只能被导入
- NJIS格式不支持导入外部NJIM模块
- 模块函数必须有返回值，确保接口明确

### 模块依赖

模块可以通过`imports`字段导入其他模块，形成模块间的依赖关系：

```json
{
  "module": "geometry",
  "imports": [
    "math.njim"    // 依赖math模块
  ],
  "exports": {
    "functions": {
      "calculateArea": {
        "body": [
          {
            "return": {
              "multiply": {
                "a": {"var": "$1"},
                "b": {"function.call": {"name": "math.square", "args": [{"var": "$2"}]}}
              }
            }
          }
        ]
      }
    }
  }
}
```
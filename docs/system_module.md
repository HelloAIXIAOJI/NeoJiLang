# 系统模块 (!system)

系统模块提供了访问操作系统功能的能力，包括文件系统操作、环境变量、进程管理等。

## 导入模块

在 NJIL 文件中导入系统模块：

```json
{
  "import": ["!system"]
}
```

## 文件系统功能

### 检查文件或目录是否存在

```json
{
  "system.fs.exists": "path/to/check"
}
```

或者使用对象格式：

```json
{
  "system.fs.exists": {
    "path": "path/to/check"
  }
}
```

返回一个布尔值，表示路径是否存在。

### 检查路径是否为文件

```json
{
  "system.fs.isFile": "path/to/check"
}
```

或者使用对象格式：

```json
{
  "system.fs.isFile": {
    "path": "path/to/check"
  }
}
```

返回一个布尔值，表示路径是否为文件。

### 检查路径是否为目录

```json
{
  "system.fs.isDir": "path/to/check"
}
```

或者使用对象格式：

```json
{
  "system.fs.isDir": {
    "path": "path/to/check"
  }
}
```

返回一个布尔值，表示路径是否为目录。

### 创建目录

```json
{
  "system.fs.mkdir": "path/to/create"
}
```

或者使用对象格式（支持递归创建）：

```json
{
  "system.fs.mkdir": {
    "path": "path/to/create",
    "recursive": true
  }
}
```

- `recursive`：（可选）如果为 `true`，则递归创建目录树，默认为 `false`。

成功时返回 `true`，失败时抛出错误。

### 删除文件或目录

```json
{
  "system.fs.remove": "path/to/remove"
}
```

或者使用对象格式（支持递归删除）：

```json
{
  "system.fs.remove": {
    "path": "path/to/remove",
    "recursive": true
  }
}
```

- `recursive`：（可选）如果为 `true`，则递归删除目录及其内容，默认为 `false`。对于删除非空目录，必须设置为 `true`。

成功时返回 `true`，失败时抛出错误。如果路径不存在，返回 `false`。

### 复制文件或目录

```json
{
  "system.fs.copy": {
    "from": "source/path",
    "to": "destination/path",
    "recursive": true
  }
}
```

- `from`：源路径
- `to`：目标路径
- `recursive`：（可选）如果为 `true`，则递归复制目录及其内容，默认为 `false`。复制目录时必须设置为 `true`。

成功时返回 `true`，失败时抛出错误。

### 移动或重命名文件或目录

```json
{
  "system.fs.move": {
    "from": "source/path",
    "to": "destination/path"
  }
}
```

- `from`：源路径
- `to`：目标路径

成功时返回 `true`，失败时抛出错误。

### 列出目录内容

```json
{
  "system.fs.list": "path/to/directory"
}
```

或者使用对象格式（支持附加选项）：

```json
{
  "system.fs.list": {
    "path": "path/to/directory",
    "includeHidden": true,
    "includeInfo": true
  }
}
```

- `includeHidden`：（可选）如果为 `true`，则包含隐藏文件（以`.`开头的文件），默认为 `false`。
- `includeInfo`：（可选）如果为 `true`，则返回详细信息而不仅仅是文件名，默认为 `false`。

当 `includeInfo` 为 `false` 时，返回文件名字符串数组。
当 `includeInfo` 为 `true` 时，返回对象数组，每个对象包含以下属性：
- `name`：文件或目录名
- `type`：类型，可能的值：`"file"`、`"directory"`、`"symlink"`、`"unknown"`
- `size`：（仅文件）文件大小（字节）
- `modified`：（如果可用）最后修改时间（Unix 时间戳，秒）

## 示例

### 检查文件是否存在并读取内容

```json
[
  {
    "import": ["!system", "!io"]
  },
  {
    "var.set": {
      "name": "file_path",
      "value": "data.txt"
    }
  },
  {
    "if": {
      "condition": {
        "system.fs.exists": {
          "path": {
            "var": "file_path"
          }
        }
      },
      "then": {
        "io.readFile": {
          "path": {
            "var": "file_path"
          }
        }
      },
      "else": {
        "println": "文件不存在"
      }
    }
  }
]
```

### 递归创建目录并写入文件

```json
[
  {
    "import": ["!system", "!io"]
  },
  {
    "var.set": {
      "name": "dir_path",
      "value": "data/logs"
    }
  },
  {
    "system.fs.mkdir": {
      "path": {
        "var": "dir_path"
      },
      "recursive": true
    }
  },
  {
    "io.writeFile": {
      "path": {
        "string.concat": [
          {
            "var": "dir_path"
          },
          "/log.txt"
        ]
      },
      "content": "日志内容"
    }
  }
]
```

### 列出目录内容并处理

```json
[
  {
    "import": ["!system"]
  },
  {
    "var.set": {
      "name": "files",
      "value": {
        "system.fs.list": {
          "path": ".",
          "includeInfo": true
        }
      }
    }
  },
  {
    "println": "当前目录中的文件:"
  },
  {
    "loop.foreach": {
      "collection": {
        "var": "files"
      },
      "var": "file",
      "body": {
        "println": {
          "string.concat": [
            {
              "var": "file.name"
            },
            " - 类型: ",
            {
              "var": "file.type"
            }
          ]
        }
      }
    }
  }
]
``` 
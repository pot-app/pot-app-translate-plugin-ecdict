# Pot-App 翻译插件模板仓库 (以 [Lingva](https://github.com/TheDavidDelta/lingva-translate) 为例)

### [English](./README_EN.md) | 简体中文

### 此仓库为模板仓库，编写插件时可以直接由此仓库创建插件仓库

## 插件编写指南

### 1. 插件仓库创建

- 以此仓库为模板创建一个新的仓库
- 仓库名为 `pot-app-translate-plugin-<插件名>`，例如 `pot-app-translate-plugin-lingva`

### 2. 插件信息配置

编辑 `info.json` 文件，修改以下字段：

- `id`：插件唯一 id，必须以`[plugin]`开头，例如 `[plugin].com.pot-app.lingva`
- `homepage`: 插件主页，填写你的仓库地址即可，例如 `https://github.com/pot-app/pot-app-translate-plugin-template`
- `display`: 插件显示名称，例如 `Lingva`
- `icon`: 插件图标，例如 `lingva.svg`
- `needs`: 插件依赖，一个数组，每个依赖为一个对象，包含以下字段：
  - `key`: 依赖 key，对应该项依赖在配置文件中的名称，例如 `requestPath`
  - `display`: 依赖显示名称，对应用户显示的名称，例如 `请求地址`
  - `type`: 组件类型 `input` | `select`
  - `options`: 选项列表(仅select组件需要)，例如 `{"engine_a":"Engina A","engine_b":"Engina B"}`
- `language`: 插件支持的语言映射，将 pot 的语言代码和插件发送请求时的语言代码一一对应

### 3. 插件编写/编译

编辑 `src/lib.rs` 实现 `translate` 函数

#### 输入参数

```rust
    text: &str, // 待翻译文本
    from: &str, // 源语言代码
    to: &str,   // 目标语言代码
    detect: &str, // 检测到的语言代码(未转换)
    needs: HashMap<String, String>, // 插件需要的其他配置信息,由info.json定义
```

#### 返回值

```rust
// 文本翻译
// 返回用Value包裹的String
  return Ok(Value::String(result));
// 词典
// 返回指定格式的json
  return Ok(json!(result));
```

词典返回 json 示例：

```json
{
  "pronunciations": [
    {
      "region": "", // 地区
      "symbol": "", // 音标
      "voice": [u8] // 语音字节数组
    }
  ],
  "explanations": [
    {
      "trait": "", // 词性
      "explains": [""] // 释义
    }
  ],
  "associations": [""], // 联想/变形
  "sentence": [
    {
      "source": "", // 原文
      "target": "" // 译文
    }
  ]
}
```

#### 测试/编译

```bash
cargo test --package plugin --lib -- tests --nocapture # 运行测试用例
cargo build --release # 编译
```

### 4. 打包 pot 插件

1. 在`target/release`目录找到`plugin.dll`(Windows)/`libplugin.dylib`(MacOS)/`libplugin.so`(Linux)文件，统一删除`lib`前缀.

2. 将`plugin.dll`/`libplugin.dylib`/`libplugin.so`文件和`info.json`以及图标文件压缩为 zip 文件。

3. 将文件重命名为`<插件id>.potext`，例如`[plugin].com.pot-app.lingva.potext`,即可得到 pot 需要的插件。

## 自动编译打包

本仓库配置了 Github Actions，可以实现推送后自动编译打包插件。

每次将仓库推送到 GitHub 之后 actions 会自动运行，将打包好的插件上传到 artifact，在 actions 页面可以下载

每次提交 Tag 之后，actions 会自动运行，将打包好的插件上传到 release，在 release 页面可以下载打包好的插件

> 注意需要在仓库设置中添加一个名为`TOKEN`的 secret，值为一个有权限的 GitHub Token，用于上传 release

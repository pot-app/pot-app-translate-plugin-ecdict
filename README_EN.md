# Pot-App Translation Plugin Template Repository (Take [Lingva](https://github.com/TheDavidDelta/lingva-translate) for example)

### English | [简体中文](./README.md)

### This repository is a template repository. You can directly create a plugin repository from this repository when writing plugins

## Plugin Writing Guide

### 1. Create a plugin repository

- Create a new repository using this repository as a template
- Named `pot-app-translate-plugin-<name>`，Eg: `pot-app-translate-plugin-lingva`

### 2. Plugin information configuration

Edit the `info.json` file and modify the following fields:

- `id`: The unique ID of the plugin, which must start with `[plugin]`, for example `[plugin].com.pot-app.lingva`
- `homepage`: The homepage of the plugin, simply fill in your repository address, for example `https://github.com/pot-app/pot-app-translate-plugin-template`
- `display`: The display name of the plugin, for example `Lingva`
- `icon`: The icon of the plugin, for example `lingva.svg`
- `needs`: Dependencies required by the plugin. It is an array where each dependency is an object that includes the following fields:
  - `key`: The key of the dependency, corresponding to its name in the configuration file. For example,`requestPath`.
  - `display`: The display name of the dependency as shown to users, for example `Request Path`.
  - `type`: Component type `input` | `select`
  - `options`: Option list(Only `select` component)， for example `{"engine_a":"Engina A","engine_b":"Engina B"}`
- `language`: Mapping between language codes used in Pot and language codes used when sending requests to plugins.

### 3. Plugin writing/compiling

Edit `src/lib.rs` to implement `translate` function

#### Input parameters

```rust
    text: &str, // text to be translated
    from: &str, // source language code
    to: &str,   // target language code
    detect: &str, // detected language code (not convert)
    needs: HashMap<String, String>, // Additional configuration information required by the plugin, defined by info.json
```

#### Return value

```rust
// Text Translation
// Returns a String wrapped in Value
  return Ok(Value::String(result));
// Dictionary
// Returns json in the specified format
  return Ok(json!(result));
```

Dictionary returns json example:

```json
{
  "pronunciations": [
    {
      "region": "",
      "symbol": "",
      "voice": [u8]
    }
  ],
  "explanations": [
    {
      "trait": "",
      "explains": [""]
    }
  ],
  "associations": [""],
  "sentence": [
    {
      "source": "",
      "target": ""
    }
  ]
}
```

#### Test/Compile

```bash
cargo test --package plugin --lib -- tests --nocapture # run the test case
cargo build --release # Compile
```

### 4. Package pot Plugin

1. Find the `plugin.dll` (Windows)/`libplugin.dylib` (MacOS)/`libplugin.so` (Linux) file in the `target/release` directory and delete the prefix `lib`.

2. Compress the `plugin.dll`/`libplugin.dylib`/`libplugin.so`, with the `info.json` and icon files, into a zip file.

3. Rename the file as `<plugin id>.potext`, for example `[plugin].com.pot-app.lingva.potext`, to obtain the plugin required by pot.

## Automatic Compilation and Package

This repository is configured with Github Actions, which allows for automatic compilation and packaging of plugins after pushing.

Every time the commit is pushed to GitHub, actions will run automatically and upload the packaged plugin to artifacts. The packaged plugin can be downloaded from the actions page.

After each tag submission, actions will also run automatically and upload the packaged plugin to releases. The packaged plugin can be downloaded from the release page.

> Please note that you need to add a secret named `TOKEN` in the repository settings. The value should be a GitHub Token with appropriate permissions, which will be used for uploading releases.

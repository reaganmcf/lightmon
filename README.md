
<p align="center">
  <img height="250px" src="https://raw.githubusercontent.com/reaganmcf/lightmon/master/assets/logo.png"/>
</p>

# lightmon
A lightweight, cross-platform, language-agnostic "run code on file change" tool, inspired by Nodemon
<p align="left">
  <img src="https://img.shields.io/static/v1?label=status&message=In%20Development&color=critical"/>
  <img src="https://img.shields.io/crates/v/lightmon"/>
  <img src="https://github.com/reaganmcf/lightmon/actions/workflows/ci.yml/badge.svg"/>
  <img src="https://shields.io/github/license/reaganmcf/lightmon"/>
</p>

###  Why lightmon over nodemon?
There are many reasons to use lightmon over nodemon: __it's faster, lighter, and can be used for all types of projects__. Not only this, but lightmon is a **drag and drop  replacement** for projects that use `nodemon` because lightman can parse existing `nodemon.json` config files.
- Note: [Parse nodemon.json config is still WIP](https://github.com/reaganmcf/lightmon/issues/3)

## Usage
```
lightmon
```
By default, `lightmon` will automatically determine what kind of files it should watch based upon your project structure. For example, if a `node_modules` folder is present in the directory, `lightmon` will run in the `node` configuration, parsing your `package.json` to infer the correct command to run.

## Supported languages

Watch patterns are the file patterns that lightmon will watch for file changes, and Exec commands are the list of commands that are executed when those events happen.

### Rust
```
lightmon rust [cargo_subcommand]? [cargo_subcommand_args]?
```

##### Watch Patterns
[`Cargo.toml`, `.rs`]
    
##### Exec Commands
By default, the `rust` configuration will set the Exec command to `cargo run` if it's a binary project, and `cargo test` if it's a library.

However, you can override this behavior by specifying any valid cargo subcommand (and any arguments). For example, if you wanted to run `cargo build --bin my_bin --all-targets`, you can run the following:
```
lightmon rust build --bin my_bin --all-targets
```

Refer to `lightmon help rust` for more information.

### Node.js
_Note: This configuration also works for React, React-Native, TypeScript, etc. i.e.: anything with a package.json!_

```
lightmon node
```

##### Watch Patterns
[`.jsx`, `.js`, `.css`, `.html`]

##### Exec Commands
If there is a package.json in the root directory, lightmon attempts to resolve the exec command in the following order:

- The value at `scripts.start`
- `node main` where main is the value of the main key in package.json (the entry point of the project).

**NOTE:** The Exec command will fallback to `node index.js` if all of the above fail.

For example, the following package.json will result in the Exec command resolving to `react-scripts start`:
```json
{
  "name": "calculator",
  "main": "index.js",
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build"
  }
}
```

### C/C++
It's very tricky to infer what the patterns and exec commands could be, so we recommend using `shell` mode with a custom script (see below).

### Shell (for unsupported languages or complicated builds)
  `lightmon shell -s <path> -w <patterns>`
  Here users can specify the path to the shell script and which file types to watch for seperated by commas.

  For example, let's say you have a python project with a file named `start.py` at the root of the project. Whenever you edit any `.py` files in the project, you want to
  re-run `python start.py`. To accomplish this, you could create a simple script called `run.sh` with the following contents:
  ```sh
  python start.py
  ```
  Now, you just run the following:
  ```
  lightmon shell -s run.sh -w .py,.ipynb
  ```

## Installation
There are many ways to install `lightmon`. We recommend using our install script as it is the fastest method.

```bash
$ curl -sSL https://raw.githubusercontent.com/reaganmcf/lightmon/master/install.sh | sh
```

But, we also support other popular package managers if you would rather use them instead:

##### `cargo`
```bash
$ cargo install lightmon
```

##### Arch AUR
```bash
$ yay -S lightmon
```

## License
`lightmon` uses the [GNU GPL v3.0](https://github.com/reaganmcf/lightmon/blob/master/LICENSE) License

### Attributions
<div>Icons made by <a href="https://www.freepik.com" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>

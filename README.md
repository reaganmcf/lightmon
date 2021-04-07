
<p align="center">
  <img height="250px" src="https://github.com/reaganmcf/lightmon/blob/master/assets/logo.png"/>
</p>

# lightmon
A lightweight, cross-platform, language-agnostic "run code on file change" tool, inspired by Nodemon
<p align="left">
  <img src="https://github.com/reaganmcf/lightmon/actions/workflows/ubuntu.yml/badge.svg"/>
  <img src="https://github.com/reaganmcf/lightmon/actions/workflows/macos.yml/badge.svg"/>
  <img src="https://github.com/reaganmcf/lightmon/actions/workflows/windows.yml/badge.svg"/>
  <img src="https://shields.io/github/license/reaganmcf/lightmon"/>
  <img src="https://img.shields.io/static/v1?label=status&message=In%20Development&color=critical"/>
</p>

## Usage
```
lightmon
```
By default, `lightmon` will automatically determine what kind of files it should watch based upon your project structure. For example, if a `node_modules` folder is present in the directory, `lightmon` will run in the `node` configuration. 

### Force language mode
You can explicitly specify which language configuration you want to use instead of letting `lightmon` infer by itself.

```
lightmon rust
```

### Supported languages
- `rust`
- `node`

While there are not many languages supported currently, we plan to have an extensive list by our first official release.

## License
`lightmon` uses the [GNU GPL v3.0](https://github.com/reaganmcf/lightmon/blob/master/LICENSE) License

### Attributions
<div>Icons made by <a href="https://www.freepik.com" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>

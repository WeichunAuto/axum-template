# 这是 Booby 的 Axum WEB 项目的基础结构模板

## 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

```bash
pipx install pre-commit
```

安装成功后运行 `pre-commit install` 即可。

## 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

## 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
```

## 安装 cargo watch

cargo watch 的作用就是 监听你的项目源码文件变化，并自动执行指定的 cargo 命令。只要保存文件，它就会自动帮你重新编译、运行或测试。

```bash
cargo install cargo-watch
```

安装成功后：

- 执行：cargo watch -x 'run' 启动项目
- 执行：APP_HOST = 127.0.0.1 APP_PORT = 8080 cargo watch -x 'run' 通过设置环境变量来启动项目

# Rundeck: a Command Line Interface for Rundeck

Rundeck (CLI) is a Command Line Interface which aim to solve a lack of tools to directly interact with a Rundeck
instance. The main goal is to run and check job's status.

![preview](./assets/preview.png)

- [1. Main features](https://github.com/Freyskeyd/Rundeck#1-main-features)
- [2. Installation](https://github.com/Freyskeyd/Rundeck#2-installation)

## 1. Main features

- List projects and jobs
- Run a job with parameters and node filters
- List executions of a project or a job
- Check executions status of a job
- Kill an execution

## 2. Installation

### From source

```bash
git clone git@github.com:Freyskeyd/Rundeck.git
cd Rundeck
cargo build --release
cp target/release/rundeck /usr/local/bin/
```

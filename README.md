# datahaikuninja/competitive-programing

My journey to competitive programing with rust and AtCoder.

## prerequiste
1. install rust.

https://www.rust-lang.org/tools/install

2. add `$HOME/.cargo/bin/` to your `$PATH` env.

3. setup [cargo-atocoder](https://github.com/tanakh/cargo-atcoder).

4. add proconio to default dependencies.

```shell
vi $HOME/Library/Application Support/cargo-atcoder.toml
```

```diff
# dependencies added to new project
[dependencies]
-# proconio = "*"
+proconio = "*"
# competitive = { git = "https://github.com/tanakh/competitive-rs.git" }

```

## cheatsheet for cargo-atcoder

```shell
# login
cargo atcoder login

# enter the contest
# e.g. $ cargo atcoder new abs
cargo atcoder new <contest-name>

# exec testcase
# e.g. $ cargo atcoder test practicea
cargo atcoder test <problem-id>

# exec binary
# e.g. $ cargo run --bin practicea
cargo run --bin <binary>

# submit answer
# e.g. $ cargo atcoder submit practicea
cargo atcoder submit <problem-id>
```

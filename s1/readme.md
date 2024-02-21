# workspace
一个workspace可以支持多个项目，只要在`cargo.toml`文件中定义members就可以了

``` shell
cargo new s1 && cd s1
cargo new tcpserver 
cargo new tcpclient
# 更新s1中的cargo.toml 
```
编译命令 
```
cargo build -p tcpserver
cargo build -p tcpclient
```

# 多个bin

## cargo.toml定义
1. 在src中建立一个lib.rs，定义所有的模块，并且在cargo.toml中定义
    ```
    [lib]
    name = "ansync-chat"
    path = "src/lib.rs"
    ```
2. 定义多个[[bin]], 每个定义个name和path
    ```
    [[bin]]
    name="client"
    path="src/cmd/client/main.rs"

    [[bin]]
    name="server"
    path="src/cmd/server/main.rs"
    ```

## 注意事项
cmd中如果只有main.rs 就不用写mod.rs了，因为这些模块已经被bin包含
## 编译
cargo build --target-dir ./target 指定目录，否则在workspace中生成target


# 问题
```
Blocking waiting for file lock on package cache
```
> rm -f ~/.cargo/.package-cache


# 包依赖

serde = {version = "1.0", features = ["derive"]}

[cargo教程](https://course.rs/cargo/intro.html)
[cargo.toml清单](https://course.rs/cargo/reference/manifest.html)

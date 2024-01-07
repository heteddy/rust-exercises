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




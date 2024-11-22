# Q&A

``` bash
Blocking waiting for file lock on package cache

rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
```

``` bash 
cargo vendor --no-delete --versioned-dirs --respect-source-config

--no-delete
  不删除上一次执行cargo vendor时留下的vendor文件夹。这样下载过的crate源码就不会再重新下载了。
  
--versioned-dirs
  给每个依赖项目录名追加以-开头的版本号后缀（例如，base64-0.5.2）。这样，不用刻意地浏览每个依赖项的Cargo.toml文件，便可知晓它们的版本信息。

--respect-source-config
  若你的工程早先就已经配置过【源码替换】[source.***]配置块，cargo将对旧配置做兼容处理。否则，旧配置就会被无视了。

```

$$
\left|\begin{matrix}
a&b&c \\
d&e&f \\
\end {matrix} \right|
$$


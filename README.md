# 这是一个demo

# 前言
如果你是一个idea用户，可以下载idea的[proto插件](https://github.com/ksprojects/protobuf-jetbrains-plugin/releases)

# 执行

````cmd
cargo build

````
执行成功的话在这个路径下```/target/debug/build/<your_project_name>-<uuid>``` 下会找到一个```out```文件夹，其中包含着生成的hello_world.rs
比如我的是```` target/debug/build/tonic_demo-d2b2af6c99ff77b8 ````

如果修改了*.proto文件，需要同步改掉.rs文件的话需要删掉该路径或者执行````cargo clean````之后重新build

将hello_world.rs复制到src文件夹下，并在main.rs中定义````mod hello_world````
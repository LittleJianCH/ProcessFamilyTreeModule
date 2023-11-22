# 请问您今天要来点生锈的内核模块吗？

最近 OS 课有个实验要求用内核模块实现在挂载模块时，根据根据传入的参数打印出对应进程的祖先进程和后代进程。这么无聊的需求如果不再整点活就太无聊了，于是 ~~Rust~~ 原神启动！

虽说网上 Linux 内核引入 Rust 的新闻已经满天飞了，但是据我实际使用下来，master 主线最新的 [c2d5304](https://github.com/torvalds/linux/tree/c2d5304e6c648ebcf653bace7e51e0e6742e46c8) 仍没有提供一个完整的 Rust 内核模块开发支持。

比如你在 rust/macros/lib.rs 里可以看到这样的注释：

``` rust
/// # Examples
///
/// ```ignore
/// use kernel::prelude::*;
///
/// module!{
///     type: MyModule,
///     name: "my_kernel_module",
///     author: "Rust for Linux Contributors",
///     description: "My very own kernel module!",
///     license: "GPL",
///     params: {
///        my_i32: i32 {
///            default: 42,
///            permissions: 0o000,
///            description: "Example of i32",
///        },
///        writeable_i32: i32 {
///            default: 42,
///            permissions: 0o644,
///            description: "Example of i32",
///        },
///    },
/// }
```

然而 rust/macros/module.rs 的实现中 `EXPECTED_KEYS` 里根本没有 `"params"`

``` rust
impl ModuleInfo {
    fn parse(it: &mut token_stream::IntoIter) -> Self {
        let mut info = ModuleInfo::default();

        const EXPECTED_KEYS: &[&str] =
            &["type", "name", "author", "description", "license", "alias"];
...
```

搜索了一圈发现 Rust-for-Linux/linux 下的 [rust 分支](https://github.com/Rust-for-Linux/linux/tree/rust) 提供了较为完整的脚手架，网上能查到的大部分资料也是根据这套代码实现的。

题外一下 rust 分支应该在实现的时候没有考虑往主线合并的问题，现在 rust-next 分支正在重新实现并和主线保持同步。

> rust was the original branch where development happened for two years before Rust support was merged into the kernel. It contains most of the abstractions that the project worked on as a prototype/showcase. Some of those will eventually land upstream, others may be reworked with feedback from upstream, and a few may be dropped if unneeded. The branch is now effectively frozen, and generally the only changes that are merged into it are intended to minimize the difference with respect to mainline. When the diff is small enough, the branch will be archived/removed. Until then, the branch is useful to see what is left to upstream, and for some downstream projects to base their work on top.

找到了正确的代码，你就可以开开心心的拉下代码，根据 `make rustavailable` 的提示安装 rust 所需的依赖啦！

在 samples/rust 文件夹下有很多可以参考的示例，这里拿一个 HelloWorld 来展示一下：

``` rust
//! HelloWorldModule

use kernel::prelude::*;

struct HelloWorldModule;

impl kernel::Module for HelloWorldModule {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let param_arg = arg.read();

        if *param_arg {
            pr_info!("Hello Rust!");
        } else {
            pr_info!("Hello World!");
        }

        Ok(Self)
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        pr_info!("Goodbye");
    }
}

module! {
    type: HelloWorldModule,
    name: "hello_world_module",
    author: "Jian",
    description: "Print Hello Rust/Hello",
    license: "GPL",
    params: {
        arg: bool {
            default: true,
            permissions: 0,
            description: "if true print Hello Rust, else print Hello World",
        },
    },
}
```

至此相信你已经能尝试用 rust 写一个简单的内核模块了。内核提供的函数接口代码在 rust/ 下，其中 rust/bindings/bindings_generated.rs 是 bindgen 自动生成的接口代码，配合上 samples 食用，相信你很快就能轻松上手了！实验的代码仓库在[这里](https://github.com/LittleJianCH/ProcessFamilyTreeModule)，你也可以拿去参考。

所以客官，请问您今天要来点生锈的内核模块吗？
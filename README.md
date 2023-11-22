# Process Family Tree Module

This a toy project for operating system course, printing all the descendan processes in a tree-like syntax. This requirement is boring, isnt it?

I write this project as a exploration of write a kernel module in rust 🦀️, and I also recorded this journey in `doc/blog.md`.

To build this module, you need to pull the `rust` branch of `Rust-for-Linux/linux` (I tested this project under `18b7491` commit). Then compile the kernel and run `make HEADERS=$(the-to-linux-source) LLVM=1`. 

Have a great day.
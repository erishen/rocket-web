#!/bin/bash

# 检查 Rust 是否已安装
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Please install Rust and try again."
    exit 1
fi

# 设置环境变量（如果有需要）
# export MY_ENV_VARIABLE=value
export APP_ENV=production

# 编译 Rust 程序
echo "Compiling Rust program..."
cargo build --release

# 检查编译是否成功
if [ $? -ne 0 ]; then
    echo "Failed to compile Rust program."
    exit 1
fi

# 运行 Rust 程序
echo "Running Rust program..."
cargo run

# 检查程序是否成功运行
if [ $? -ne 0 ]; then
    echo "Failed to run Rust program."
    exit 1
else
    echo "Rust program ran successfully."
fi

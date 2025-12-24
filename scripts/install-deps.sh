#!/bin/bash
# 安装 Tauri 在 Linux 上的系统依赖

set -e

echo "📦 正在安装 Tauri 系统依赖..."

# 检测 Linux 发行版
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
else
    echo "❌ 无法检测 Linux 发行版"
    exit 1
fi

case $OS in
    ubuntu|debian)
        echo "检测到 Ubuntu/Debian 系统"
        sudo apt-get update
        sudo apt-get install -y \
            libwebkit2gtk-4.1-dev \
            libgtk-3-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev \
            build-essential \
            curl \
            wget \
            file
        ;;
    fedora|rhel|centos)
        echo "检测到 Fedora/RHEL/CentOS 系统"
        sudo dnf install -y \
            webkit2gtk4.1-devel \
            gtk3-devel \
            libappindicator-gtk3-devel \
            librsvg2-devel \
            gcc \
            gcc-c++ \
            curl \
            wget \
            file
        ;;
    arch|manjaro)
        echo "检测到 Arch/Manjaro 系统"
        sudo pacman -Sy --needed \
            webkit2gtk-4.1 \
            gtk3 \
            libappindicator-gtk3 \
            librsvg \
            base-devel \
            curl \
            wget \
            file
        ;;
    *)
        echo "❌ 不支持的发行版: $OS"
        echo "请参考 Tauri 官方文档: https://tauri.app/v1/guides/getting-started/prerequisites"
        exit 1
        ;;
esac

echo "✅ 系统依赖安装完成!"
echo ""
echo "现在可以运行:"
echo "  npm install"
echo "  cargo check"

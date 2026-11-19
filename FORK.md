# Niri 侧笺：分支说明

Niri 侧笺是上游 [0atxl/stickies](https://github.com/0atxl/stickies) 0.10.3 的**非官方简体中文分支**，与上游项目无隶属关系。目标是一个只面向简体中文、贴边使用、可自由选屏的日常便签工具。

## 与上游的主要差异

- 界面、日期、CLI 描述、错误提示和配置错误直接写简体中文，不再包含运行时语言切换或定时翻译。
- 设置里可以选择贴边屏幕（GDK connector，如 `HDMI-A-1`），也可以选择上/下/左/右边缘。
- 收起态不绘制任何点/线触发图标；整条所选屏幕边缘都是不可见热区。
- 标签固定长度，初始居中；数量超过边缘容量后动态重叠，每个标签保留最小可见宽度。
- 标签为文档式外观：细内缘便签色线、当前标签高亮、非当前标签降低透明度、完整标题 tooltip。
- 删除应用内「开机自启」开关和 `autostart` 模块；开机启动统一交给 niri：

```kdl
spawn-at-startup "/usr/bin/niri-side-note"
```

## 命名与兼容

- 产品名：**Niri 侧笺**（Niri Side Note）。
- 命令/包名：`niri-side-note`；源码里的 Cargo 包名、D-Bus 应用 ID、配置与数据目录仍沿用 `stickies`，以免覆盖上游包或丢失旧笔记。
- 配置：`~/.config/stickies/config.toml`；数据：`~/.local/share/stickies/notes.db`。

## 构建与安装

```bash
cargo build --release --locked
install -Dm755 target/release/stickies ~/.local/bin/niri-side-note
```

niri 自启和启动器均指向 `niri-side-note`；不要与上游 `/usr/bin/stickies` 混用。

## 已知限制

- 只支持 Wayland + layer-shell v4；GNOME Wayland 不可用。
- 中文需要系统 CJK 字体（如 `noto-fonts-cjk`）；没有字体时中文会缺字形。
- 设置里的屏幕列表在打开时枚举；运行中热插拔显示器需要重开设置。
- 真实鼠标 hover 与重叠标签坐标命中仍以真机测试为准。

## 许可证

上游与本分支均采用 **MPL-2.0**；完整许可证见 [LICENSE](LICENSE)。本分支的修改与打包脚本以同一许可证公开，完整对应源码即本仓库。

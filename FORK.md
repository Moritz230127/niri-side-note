# 本地分支说明（中文单语）

本分支是在上游 Stickies 0.10.3 基础上做的本地二次开发，目标是一个只面向简体中文、贴边使用、可自由选屏的日常便签工具。

## 与上游的主要差异

- 界面、日期、CLI 描述、错误提示和配置错误直接写简体中文，不再包含运行时语言切换或定时翻译。
- 设置里可以选择贴边屏幕（GDK connector，如 `HDMI-A-1`），也可以选择上/下/左/右边缘。
- 收起态不绘制任何点/线触发图标；整条所选屏幕边缘都是不可见热区。
- 标签固定长度，初始居中；数量超过边缘容量后动态重叠，每个标签保留最小可见宽度。
- 标签为文档式外观：细内缘便签色线、当前标签高亮、非当前标签降低透明度、完整标题 tooltip。
- 删除应用内「开机自启」开关和 `autostart` 模块；开机启动统一交给 niri：

```kdl
spawn-at-startup "/home/Arch/.local/bin/stickies"
```

## 配置兼容

- `Config` 不再拒绝未知键；旧配置中的 `launch_at_login`、`marker_style`、`visible_notes` 会被忽略并在下次保存时移除。
- `marker_offset` 仍保留，表示沿边缘的位置（0..1），键名为兼容旧配置未改名。

## 构建与安装

```bash
cargo build --release --locked
install -Dm755 target/release/stickies ~/.local/bin/stickies
```

niri 自启和启动器均指向 `~/.local/bin/stickies`；不要使用 `/usr/bin/stickies`（上游旧版）。

## 已知限制

- 只支持 Wayland + layer-shell v4；GNOME Wayland 不可用。
- 中文需要系统 CJK 字体（如 `noto-fonts-cjk`）；没有字体时中文会缺字形。
- 设置里的屏幕列表在打开时枚举；运行中热插拔显示器需要重开设置。
- 真实鼠标 hover 与重叠标签坐标命中仍以真机测试为准。

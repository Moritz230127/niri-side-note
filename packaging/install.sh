#!/bin/sh
# 便携包安装脚本：只写入用户目录，不修改系统包，也不会启用开机自启。
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
BIN_DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"
DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"
APP_DIR="$DATA_HOME/applications"
ICON_DIR="$DATA_HOME/icons/hicolor/128x128/apps"
BIN="$BIN_DIR/niri-side-note"
DESKTOP="$APP_DIR/niri-side-note.desktop"

install -Dm755 "$ROOT/bin/niri-side-note" "$BIN"
install -Dm644 "$ROOT/share/applications/niri-side-note.desktop" "$DESKTOP"
install -Dm644 "$ROOT/share/icons/hicolor/128x128/apps/dev.stickies.Stickies.png" \
    "$ICON_DIR/dev.stickies.Stickies.png"

# 便携安装写入绝对路径，避免桌面会话没有把 ~/.local/bin 放进 PATH。
sed -i "s|^TryExec=.*|TryExec=$BIN|; s|^Exec=.*|Exec=$BIN|" "$DESKTOP"

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$APP_DIR" >/dev/null 2>&1 || true
fi

printf '已安装：%s\n' "$BIN"
printf '桌面入口：%s\n' "$DESKTOP"
printf '直接运行：%s\n' "$BIN"
printf '开机自启未修改；如需 niri 自启，请手动添加 spawn-at-startup。\n'

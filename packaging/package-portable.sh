#!/usr/bin/env bash
# 构建 rodriguez pacman 仓库使用的便携发行包：
#   dist/portable/niri-side-note-<版本>-linux-x86_64.tar.gz
#
# 目录布局与现有 Release 资产保持一致：
#   bin/niri-side-note
#   share/applications/niri-side-note.desktop
#   share/icons/hicolor/128x128/apps/dev.stickies.Stickies.png
#   share/doc/niri-side-note/README.md
#   share/licenses/niri-side-note/{LICENSE,NOTICE,fonts/*}
#
# 用法：
#   ./packaging/package-portable.sh          # 需要系统已安装 gtk4-layer-shell
#   STRIP=0 ./packaging/package-portable.sh  # 保留调试符号（默认 strip）
set -euo pipefail

cd "$(dirname "$0")/.."
ROOT=$PWD
VERSION=$(python3 -c "import tomllib;print(tomllib.load(open('Cargo.toml','rb'))['package']['version'])")
ARCH=${ARCH:-x86_64}
NAME="niri-side-note-$VERSION-linux-$ARCH"
OUT=${OUT:-$ROOT/dist/portable}
STAGE="$OUT/$NAME"
CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$ROOT/target}

cargo build --release --locked
BIN="$CARGO_TARGET_DIR/release/stickies"
test -x "$BIN"

rm -rf "$STAGE"
install -d "$STAGE/bin" \
  "$STAGE/share/applications" \
  "$STAGE/share/icons/hicolor/128x128/apps" \
  "$STAGE/share/doc/niri-side-note" \
  "$STAGE/share/licenses/niri-side-note/fonts"

install -m755 "$BIN" "$STAGE/bin/niri-side-note"
if [ "${STRIP:-1}" = 1 ]; then
  strip --strip-all "$STAGE/bin/niri-side-note"
fi
install -m644 packaging/niri-side-note.desktop "$STAGE/share/applications/niri-side-note.desktop"
install -m644 data/dev.stickies.Stickies.png \
  "$STAGE/share/icons/hicolor/128x128/apps/dev.stickies.Stickies.png"
install -m644 README.md "$STAGE/share/doc/niri-side-note/README.md"
install -m644 LICENSE "$STAGE/share/licenses/niri-side-note/LICENSE"
install -m644 NOTICE "$STAGE/share/licenses/niri-side-note/NOTICE"
install -m644 assets/fonts/*-LICENSE.txt "$STAGE/share/licenses/niri-side-note/fonts/"
install -m644 packaging/licenses/gtk4-layer-shell-MIT.txt \
  "$STAGE/share/licenses/niri-side-note/fonts/gtk4-layer-shell-MIT.txt"

TARBALL="$OUT/$NAME.tar.gz"
tar --sort=name --owner=0 --group=0 --numeric-owner -czf "$TARBALL" -C "$OUT" "$NAME"
SHA256=$(sha256sum "$TARBALL" | awk '{print $1}')
printf '%s  %s\n' "$SHA256" "$(basename "$TARBALL")" > "$TARBALL.sha256"

echo
echo "版本:   $VERSION"
echo "产物:   $TARBALL"
echo "SHA256: $SHA256"
echo
echo "发布后由 rodriguez workflow 自动打包；本地验证可执行："
echo "  tar -xOf '$TARBALL' '$NAME/bin/niri-side-note' >/dev/null && echo OK"

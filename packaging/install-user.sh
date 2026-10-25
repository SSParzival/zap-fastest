#!/usr/bin/env bash
# Installs a source build for the current user: the binary, the icon, and a
# launcher entry that opens ZapFastest with no terminal.
#
# Usage: packaging/install-user.sh [binary] [prefix]
#   binary  the built executable (default: target/release/zap-fastest)
#   prefix  the install prefix   (default: $PREFIX, else ~/.local with data
#           in $XDG_DATA_HOME when it is set)
set -euo pipefail

here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo=$(dirname -- "$here")

binary=${1:-""}
if [[ -z "$binary" ]]; then
  if [[ -x "$repo/target/release/zap-fastest" ]]; then
    binary="$repo/target/release/zap-fastest"
  elif [[ -x "$repo/target/release/zap-faster" ]]; then
    binary="$repo/target/release/zap-faster"
  else
    binary="$repo/target/release/zap-fastest"
  fi
fi

prefix=${2:-"${PREFIX:-}"}
if [[ -n "$prefix" ]]; then
  mkdir -p "$prefix"
  prefix=$(cd -- "$prefix" && pwd)
  data_dir="$prefix/share"
else
  prefix="$HOME/.local"
  data_dir="${XDG_DATA_HOME:-$prefix/share}"
fi
bin_dir="$prefix/bin"
apps_dir="$data_dir/applications"
icons_dir="$data_dir/icons/hicolor/scalable/apps"

if [[ ! -x "$binary" ]]; then
  echo "No built binary at $binary; build it first: cargo build --release" >&2
  exit 1
fi

installed="$bin_dir/zap-fastest"
mkdir -p "$bin_dir" "$icons_dir" "$apps_dir"
install -Dm755 "$binary" "$installed"

if [[ -f "$here/icons/zap-fastest.svg" ]]; then
  install -Dm644 "$here/icons/zap-fastest.svg" "$icons_dir/zap-fastest.svg"
elif [[ -f "$here/icons/zap-faster.svg" ]]; then
  install -Dm644 "$here/icons/zap-faster.svg" "$icons_dir/zap-faster.svg"
fi

desktop_src="$here/applications/zap-fastest.desktop"
if [[ ! -f "$desktop_src" ]]; then
  desktop_src="$here/applications/zap-faster.desktop"
fi

EXEC_PATH="$installed" awk '
  function quote(path,   out, i, c) {
    out = "\""
    for (i = 1; i <= length(path); i++) {
      c = substr(path, i, 1)
      if (c == "\"" || c == "`" || c == "$" || c == "\\") out = out "\\"
      if (c == "%") out = out "%"
      out = out c
    }
    return out "\""
  }
  /^Exec=zap-fast(est|er)?( |$)/ {
    path = ENVIRON["EXEC_PATH"]
    if (path !~ /^\/[A-Za-z0-9_.\/-]+$/) path = quote(path)
    sub(/^Exec=zap-fast(est|er)?/, "Exec=" path)
    print
    next
  }
  { print }
' "$desktop_src" > "$apps_dir/zap-fastest.desktop"

if command -v desktop-file-validate >/dev/null 2>&1 && [[ "$installed" != *\\* ]]; then
  desktop-file-validate "$apps_dir/zap-fastest.desktop" || true
fi
if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$apps_dir" 2>/dev/null || true
fi

echo "Installed $installed"
echo "Installed $apps_dir/zap-fastest.desktop"
echo "ZapFastest is now installed and accessible from the application launcher."

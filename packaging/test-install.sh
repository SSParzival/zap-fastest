#!/usr/bin/env bash
# Usage: bash packaging/test-install.sh ubuntu:24.04 native-packages-output
# The compiler runs on the host so it cannot supply missing runtime libraries.
set -euo pipefail
image=${1:?Supply an Ubuntu, Debian or Fedora container image}
packages=$(realpath "${2:?Supply a native-packages output directory}")
case "$(uname -m)" in
  x86_64) target=linux-amd64 ;;
  aarch64) target=linux-arm64 ;;
  *) echo 'Unsupported test architecture' >&2; exit 1 ;;
esac
case "$image" in
  ubuntu:*|debian:*) format=deb ;;
  fedora:*) format=rpm ;;
  *) echo 'Unsupported test distribution' >&2; exit 1 ;;
esac
package_dir="$packages/packages/$target/$format"
test -d "$package_dir"
script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
checks=$(mktemp -d)
trap 'rm -rf -- "$checks"' EXIT
cc -std=c99 -Wall -Wextra -Werror "$script_dir/check-runtime-libs.c" -ldl -o "$checks/check-runtime-libs"
docker run --rm \
  --volume "$package_dir:/packages:ro" \
  --volume "$checks:/checks:ro" \
  --env "FORMAT=$format" "$image" sh -ec '
    set -- /packages/*."$FORMAT"
    test "$#" -eq 1
    test -f "$1"
    mkdir -p /root/.config/zap-fastest
    printf "%s\n" "preserve-existing-settings" > /root/.config/zap-fastest/fixture
    if [ "$FORMAT" = deb ]; then
      apt-get update
      DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "$1"
      dpkg-query -W zap-fastest
    else
      dnf install -y --setopt=install_weak_deps=False "$1"
      rpm -q zap-fastest
    fi
    zap-fastest --version
    /checks/check-runtime-libs
    test -s /usr/share/applications/zap-fastest.desktop
    test -s /usr/share/icons/hicolor/scalable/apps/zap-fastest.svg
    grep -qx "Icon=zap-fastest" /usr/share/applications/zap-fastest.desktop
    grep -qx "StartupWMClass=zap-fastest" /usr/share/applications/zap-fastest.desktop
    test -s /usr/share/zap-fastest/omarchy/zap-fastest.json.tpl
    test -x /usr/share/zap-fastest/omarchy/zap-fastest-theme
    if [ "$FORMAT" = deb ]; then apt-get remove -y zap-fastest; else dnf remove -y zap-fastest; fi
    test ! -e /usr/bin/zap-fastest
    test ! -e /usr/share/applications/zap-fastest.desktop
    test ! -e /usr/share/icons/hicolor/scalable/apps/zap-fastest.svg
    test ! -e /usr/share/zap-fastest/omarchy/zap-fastest.json.tpl
    test ! -e /usr/share/zap-fastest/omarchy/zap-fastest-theme
    test "$(cat /root/.config/zap-fastest/fixture)" = preserve-existing-settings
  '

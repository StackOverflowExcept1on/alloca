#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: scripts/prebuilt.sh <target>" >&2
  exit 2
fi

readonly target="$1"

cargo clean

package_id="$(cargo metadata --format-version=1 | jq --raw-output '.resolve.root')"
out_dir="$({
  cargo build \
    --release \
    --target "$target" \
    --no-default-features \
    --features compile-alloca \
    --message-format=json
} | jq \
  --raw-output \
  --arg package_id "$package_id" \
  'select(.reason == "build-script-executed" and .package_id == $package_id) | .out_dir' \
  | tail -n 1)"

if [[ -z "$out_dir" ]]; then
  echo "error: Cargo did not report alloca build script output directory" >&2
  exit 1
fi

printf '%s\n' "$out_dir"

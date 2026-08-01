#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/cargo.sh <command> [cargo arguments...]

Run every feature combination in both debug and release profiles.

Commands:
  build    Build every configuration
  test     Test every configuration
  clippy   Lint every configuration with warnings denied

Options:
  -h, --help  Show this help message

Additional arguments are passed to Cargo, for example:
  scripts/cargo.sh build --target x86_64-unknown-linux-gnu
EOF
}

if [[ $# -eq 0 ]]; then
  usage >&2
  exit 2
fi

if [[ "$1" == "-h" || "$1" == "--help" ]]; then
  usage
  exit 0
fi

readonly cargo_command="$1"
shift

case "$cargo_command" in
  build | test | clippy) ;;
  *)
    echo "error: expected build, test, or clippy; got '$cargo_command'" >&2
    echo >&2
    usage >&2
    exit 2
    ;;
esac

no_test_targets=(
  "wasm32-unknown-unknown"
  "wasm32v1-none"
)
no_all_targets=(
  "wasm32v1-none"
)

target_is_in() {
  local target="$1"
  shift

  local candidate
  for candidate in "$@"; do
    if [[ "$target" == "$candidate" ]]; then
      return 0
    fi
  done

  return 1
}

target=""
expect_target=false
for arg in "$@"; do
  if $expect_target; then
    target="$arg"
    expect_target=false
  elif [[ "$arg" == "--target" ]]; then
    expect_target=true
  elif [[ "$arg" == --target=* ]]; then
    target="${arg#--target=}"
  fi
done

if [[ "$cargo_command" == "test" ]] && target_is_in "$target" "${no_test_targets[@]}"; then
  echo "[+] Skipping Cargo test for $target: target has no test harness"
  exit 0
fi

clippy_all_targets=false
if [[ "$cargo_command" == "clippy" ]] && ! target_is_in "$target" "${no_all_targets[@]}"; then
  clippy_all_targets=true
fi

profiles=("debug" "release")
feature_sets=(
  "default"
  "compile-alloca"
  "stack-clash-protection"
  "stack-protector"
  "stack-clash-protection,stack-protector"
)

for profile in "${profiles[@]}"; do
  for features in "${feature_sets[@]}"; do
    command=(cargo "$cargo_command" "$@")

    if $clippy_all_targets; then
      command+=(--all-targets)
    fi

    if [[ "$profile" == "release" ]]; then
      command+=(--release)
    fi

    if [[ "$features" != "default" ]]; then
      command+=(--no-default-features --features "$features")
    fi

    if [[ "$cargo_command" == "clippy" ]]; then
      command+=(-- -D warnings)
    fi

    printf '[+] Cargo %s (profile: %s, features: %s)\n' "$cargo_command" "$profile" "$features"
    printf '   '
    printf ' %q' "${command[@]}"
    printf '\n'

    "${command[@]}"
  done
done

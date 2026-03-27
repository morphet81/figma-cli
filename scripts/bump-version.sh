#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

PUSH=0
LEVEL=""
ARGS=()
for a in "$@"; do
  if [[ "$a" == "--push" ]]; then
    PUSH=1
  else
    ARGS+=("$a")
  fi
done
set -- "${ARGS[@]}"

usage_err() {
  echo "Usage: $0 [--push] major|minor|patch" >&2
  echo "  --push  push branch and tag to origin without prompting" >&2
  exit 1
}

for a in "$@"; do
  case "$a" in
    major | minor | patch)
      if [[ -n "$LEVEL" ]]; then
        echo "Specify only one of major, minor, patch." >&2
        usage_err
      fi
      LEVEL="$a"
      ;;
    -h | --help)
      echo "Usage: $0 [--push] major|minor|patch"
      echo "  --push  push branch and tag to origin without prompting"
      exit 0
      ;;
    *)
      echo "Unknown argument: $a" >&2
      usage_err
      ;;
  esac
done

if [[ -z "$LEVEL" ]]; then
  usage_err
fi

if ! command -v cargo &>/dev/null; then
  echo "cargo not found in PATH." >&2
  exit 1
fi

if ! cargo set-version --help >/dev/null 2>&1; then
  echo "cargo-edit is required so that \`cargo set-version\` is available." >&2
  echo "Install: cargo install cargo-edit" >&2
  exit 1
fi

cargo set-version --bump "$LEVEL"

version="$(cargo pkgid | sed 's/.*#//' | sed 's/^[^@]*@//')"
tag="v${version}"

git add Cargo.toml Cargo.lock
git commit -m "Release ${tag}"
git tag -a "${tag}" -m "Release ${tag}"

branch="$(git rev-parse --abbrev-ref HEAD)"

echo "Created commit and annotated tag ${tag}."

do_push() {
  git push origin "${branch}"
  git push origin "${tag}"
  echo "Pushed ${branch} and ${tag}. The release workflow will run for this tag."
}

if [[ "$PUSH" -eq 1 ]]; then
  do_push
elif [[ -t 0 ]]; then
  read -r -p "Push branch and tag to origin? [y/N] " answer
  norm=$(echo "${answer}" | tr '[:upper:]' '[:lower:]')
  if [[ "${norm}" == "y" || "${norm}" == "yes" ]]; then
    do_push
  else
    echo "Push skipped. When ready:"
    echo "  git push origin ${branch}"
    echo "  git push origin ${tag}"
  fi
else
  echo "Push skipped (non-interactive). Run:"
  echo "  git push origin ${branch}"
  echo "  git push origin ${tag}"
fi

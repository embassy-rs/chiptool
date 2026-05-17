#!/bin/bash
## on pull_request

set -euxo pipefail

export RUSTUP_HOME=/ci/cache/rustup
export CARGO_HOME=/ci/cache/cargo
export CARGO_TARGET_DIR=/ci/cache/target

hashtime restore /ci/cache/filetime.json || true
hashtime save /ci/cache/filetime.json

# Fetch enough history to compute the merge-base with main.
git remote add upstream https://github.com/embassy-rs/chiptool
git fetch --depth 50 upstream main
BASE=$(git merge-base HEAD upstream/main)
BASE=0c94b6c4c43712a455d687d34c7200909203f2d9
HEAD_SHA=$(git rev-parse HEAD)

# Reuse cached SVDs clone across CI runs.
mv /ci/cache/chiptool-test-svds tests/svds || true

run_test() {
    local out=$1
    rm -rf tests/output
    cargo test --test svd2ir --release
    rm -rf "$out"
    mv tests/output "$out"
}

# Generate YAMLs for HEAD.
run_test /tmp/head

# Generate YAMLs for the merge base.
git -c advice.detachedHead=false checkout "$BASE"
run_test /tmp/base
git checkout -

# Move cached SVDs back.
mv tests/svds /ci/cache/chiptool-test-svds

# Produce a colored HTML diff.
mkdir -p /ci/artifacts
(
    cd /tmp
    # `--no-index` lets us diff two trees outside any repo.
    # exit code 1 just means "differences found"; that's fine.
    git --no-pager diff --no-index --color=always base head > /tmp/diff.ansi || true
)
aha --black < /tmp/diff.ansi > /ci/artifacts/diff.html

# Also publish the raw YAML trees so reviewers can browse / download them.
cp -r /tmp/head /ci/artifacts/head
cp -r /tmp/base /ci/artifacts/base

JOB_ID=$(jq -r .id < /ci/job.json)
cat > /ci/comment.md <<EOF
chiptool svd2ir diff for commit \`$HEAD_SHA\` vs base \`$BASE\`:

- [diff.html](https://ci.embassy.dev/jobs/$JOB_ID/artifacts/diff.html)
- [head YAMLs](https://ci.embassy.dev/jobs/$JOB_ID/artifacts/head/)
- [base YAMLs](https://ci.embassy.dev/jobs/$JOB_ID/artifacts/base/)
EOF

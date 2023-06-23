#!/usr/bin/env bash

HOOKS_DIR="$(git rev-parse --git-path hooks)"

# pre-push

PREPUSH="#!/usr/bin/env bash
just check
just fmt
"

echo "$PREPUSH" > "$HOOKS_DIR/pre-push"
chmod +x "$HOOKS_DIR/pre-push"

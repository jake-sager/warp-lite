# Warp Lite

Fork of Warp aimed at getting back to the core terminal: fast local shells, tabs, panes, blocks, selection, scrollback, command history, etc. Gets rid of all the AI stuff, cloud accounts, workspace settings, enterprise features, etc. Basically, trying to get back warp from a few years ago!

## What Has Changed

So far this fork has been reshaped around a local-only `warp-oss` build:

- AI, agent, MCP, cloud conversation, and coding-agent paths have been stubbed or removed from the runnable app path.
- Account, sign-in, onboarding, referrals, teams, billing, update, telemetry, crash reporting, and production service configuration have been disconnected from startup.
- Bootstrap and run scripts no longer require Google Cloud auth for the local OSS build path.
- The macOS app menus have been pruned down toward terminal-owned actions.
- Settings navigation has been collapsed to local/core settings.
- Startup now bypasses auth/onboarding and opens directly into a local terminal workspace.
- Local settings and local history/storage are intentionally preserved.

There is still cleanup left. Some inert compatibility models remain because the terminal UI still has read-only references into old product modules. Those should be deleted as the surrounding callers are removed.

## Local Data

Warp Lite keeps local data as the preservation path:

- `settings.toml` remains the main portable settings file.
- private local preferences remain in the local platform store / `user_preferences.json`.
- shell history, command history, restored sessions, tabs, panes, and scrollback are intended to remain local.

## Building the Repo Locally

To build and run Warp Lite from source:

```bash
./script/bootstrap   # platform-specific setup
./script/run         # build and run Warp Lite
./script/presubmit   # fmt, clippy, and tests
```

Useful focused checks while working on the app:

```bash
cargo check -p warp --bin warp-oss
cargo build -p warp --bin warp-oss
./script/run --dont-open
```

The macOS debug bundle is written to:

```text
target/debug/bundle/osx/WarpOss.app
```

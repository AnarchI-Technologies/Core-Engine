# GUI Shell

This folder contains a Tauri-ready GUI prototype for AnarchI Core.

Current state:

- Static HTML/CSS/JS product shell.
- Models the screens the Rust engine needs to feed.
- Intended to be wrapped by Tauri once Rust tooling is installed.

Open `app/index.html` directly for a local preview.

Future Tauri command bindings:

- `engine_plan(request)` -> calls `anarchi_core::orchestrator::plan`.
- `engine_apply_trim(plan_id)` -> runs approved trim actions.
- `engine_launch(plan_id)` -> launches the target.
- `plugins_install(package_id)` -> validates and installs a signed marketplace plugin.

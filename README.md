# HOANGSA

> A lean 3-phase (menu → prepare → cook) context engineering system for Claude Code.

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![npm version](https://img.shields.io/npm/v/hoangsa-cc.svg)
![Claude Code](https://img.shields.io/badge/Claude_Code-compatible-blueviolet.svg)
![Built with Rust](https://img.shields.io/badge/Built_with-Rust-orange.svg)
![Node.js](https://img.shields.io/badge/Node.js-18+-green.svg)

---

## What is HOANGSA?

HOANGSA is a context engineering system for [Claude Code](https://docs.anthropic.com/en/docs/claude-code). It solves a fundamental problem: **Claude's output quality degrades as the context window fills up.**

The fix is structural. HOANGSA splits work into discrete tasks. Each task runs in a fresh context window with only the files it actually needs. The result is consistent, high-quality output across arbitrarily large projects.

The core pipeline has three phases:

| Phase | Command | Output |
|-------|---------|--------|
| Design | `/hoangsa:menu` | DESIGN-SPEC + TEST-SPEC |
| Plan | `/hoangsa:prepare` | Executable task DAG (`plan.json`) |
| Execute | `/hoangsa:cook` | Working code, wave by wave |

The orchestrator never writes code. It dispatches workers, each with a bounded context, and assembles results.

---

## Features

**Context Engineering** — Each worker task runs in a fresh context window. The plan's `context_pointers` tell each worker exactly which files to read — no more, no less. This is the core value proposition.

**Spec-Driven Development** — Every feature starts with a DESIGN-SPEC (requirements, interfaces, acceptance criteria) and TEST-SPEC (test cases, coverage targets). Workers implement against specs, not vague instructions.

**DAG-Based Execution** — Tasks are organized as a directed acyclic graph with dependency resolution. Independent tasks execute in parallel waves, dependent tasks execute sequentially. No unnecessary serialization.

**Cross-Layer Bug Tracing** — `/hoangsa:fix` spawns a research agent that traces bugs across FE/BE/API/DB boundaries. A frontend bug might originate in a backend API contract — HOANGSA finds the real root cause before touching any code.

**8-Dimension Codebase Audit** — `/hoangsa:audit` scans for code smells, security vulnerabilities, performance bottlenecks, tech debt, test coverage gaps, dependency risks, architectural violations, and documentation gaps.

**Task Manager Integration** — Paste a task link (ClickUp, Asana) anywhere in the workflow. HOANGSA pulls task details as context and syncs results back (status updates, comments, reports) after work completes.

**GitNexus Code Intelligence** — Built-in call graph analysis. Run impact analysis before any edit, perform safe renames across the entire codebase, and trace full execution flows from entry point to leaf function.

**Multi-Profile Model Selection** — Switch between quality, balanced, and budget model profiles to match task requirements and cost constraints.

---

## Quick Start

```bash
npx hoangsa-cc          # Install HOANGSA into your Claude Code environment
/hoangsa:init           # Initialize project — detect codebase, set preferences
/hoangsa:menu           # Design your first task
```

After `/hoangsa:menu` completes, follow with `/hoangsa:prepare` to generate a plan, then `/hoangsa:cook` to execute it.

---

## Installation

Prerequisites: **Node.js 18+** and the **[Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code)**

```bash
# Interactive — asks whether to install globally or locally
npx hoangsa-cc

# Install to ~/.claude/ — available in all projects
npx hoangsa-cc --global

# Install to .claude/ — this project only
npx hoangsa-cc --local

# Remove HOANGSA
npx hoangsa-cc --uninstall

# Install to a custom config directory
npx hoangsa-cc --config-dir <path>
```

| Flag | Short | Description |
|------|-------|-------------|
| `--global` | `-g` | Install to `~/.claude/` (all projects) |
| `--local` | `-l` | Install to `.claude/` (this project only) |
| `--uninstall` | `-u` | Remove HOANGSA |
| `--config-dir` | | Use a custom config directory path |

---

## Workflow

```
idea  →  /menu      Design    →  DESIGN-SPEC + TEST-SPEC
      →  /prepare   Plan      →  Executable task DAG (plan.json)
      →  /cook      Execute   →  Wave-by-wave, fresh context per task
      →  /taste     Test      →  Acceptance tests per task
      →  /plate     Commit    →  Conventional commit message
      →  /serve     Sync      →  Bidirectional task manager sync
```

**Design (`/menu`)** — Interview the user about requirements. Produce a structured DESIGN-SPEC with interfaces and acceptance criteria, plus a TEST-SPEC with test cases and coverage targets.

**Plan (`/prepare`)** — Parse the specs and generate `plan.json`: a DAG of tasks, each with an assigned worker, bounded file list (`context_pointers`), and explicit dependency edges.

**Execute (`/cook`)** — Walk the DAG wave by wave. Dispatch each worker with its context. Independent tasks in the same wave can run in parallel. Aggregate results before advancing.

**Test (`/taste`)** — Run the acceptance tests defined in TEST-SPEC. Report pass/fail per task. Block the pipeline on failures.

**Commit (`/plate`)** — Stage changes and generate a conventional commit message from the completed work.

**Sync (`/serve`)** — Push status updates, comments, and artifacts back to the linked task manager.

---

## Commands

### Core Workflow

| Command | Description |
|---------|-------------|
| `/hoangsa:menu` | Design — from idea to DESIGN-SPEC + TEST-SPEC |
| `/hoangsa:prepare` | Plan — convert specs to an executable task DAG |
| `/hoangsa:cook` | Execute — wave-by-wave with fresh context per task |
| `/hoangsa:taste` | Test — run acceptance tests per task |
| `/hoangsa:plate` | Commit — generate and apply a conventional commit message |
| `/hoangsa:serve` | Sync — bidirectional sync with connected task manager |

### Specialized

| Command | Description |
|---------|-------------|
| `/hoangsa:fix` | Hotfix — cross-layer root cause tracing + minimal targeted fix |
| `/hoangsa:audit` | Audit — 8-dimension codebase scan (security, debt, coverage, etc.) |
| `/hoangsa:research` | Research — codebase analysis combined with external research |

### Utility

| Command | Description |
|---------|-------------|
| `/hoangsa:init` | Initialize — detect codebase, configure preferences, first-time setup |
| `/hoangsa:check` | Status — show current session progress and pending tasks |
| `/hoangsa:index` | Index — rebuild GitNexus code intelligence graph |
| `/hoangsa:update` | Update — upgrade HOANGSA to the latest version |
| `/hoangsa:help` | Help — show all available commands |

---

## Configuration

HOANGSA stores project configuration in `.hoangsa/config.json`.

```json
{
  "lang": "en",
  "spec_lang": "en",
  "tech_stack": ["typescript", "react", "postgres"],
  "review_style": "strict",
  "model_profile": "balanced",
  "task_manager": {
    "provider": "clickup",
    "token": "<your-token>"
  }
}
```

### Preferences

| Key | Values | Description |
|-----|--------|-------------|
| `lang` | `en`, `vi` | Language for orchestrator output |
| `spec_lang` | `en`, `vi` | Language for generated specs |
| `tech_stack` | array | Project technology stack (used to tune worker instructions) |
| `review_style` | `strict`, `balanced`, `light` | Code review thoroughness |

### Model Profiles

Select a profile to control the model used at each role:

| Profile | Worker | Designer | Reviewer |
|---------|--------|----------|----------|
| `quality` | claude-opus | claude-opus | claude-opus |
| `balanced` | claude-sonnet | claude-opus | claude-sonnet |
| `budget` | claude-haiku | claude-sonnet | claude-haiku |

Switch profiles with `/hoangsa:init` or by editing `model_profile` in `config.json`.

### Task Manager Integration

| Provider | How to connect |
|----------|---------------|
| ClickUp | Paste a ClickUp task URL |
| Asana | Paste an Asana task URL |

HOANGSA fetches task details as additional context and writes results back on `/hoangsa:serve`.

---

## Architecture

### Project Structure

```
hoangsa/
├── cli/                        # Rust CLI (hoangsa-cli)
│   └── src/
│       ├── cmd/                # 13 command modules
│       │   ├── config.rs       # Config read/write
│       │   ├── context.rs      # Context pointer resolution
│       │   ├── dag.rs          # DAG traversal and wave scheduling
│       │   ├── hook.rs         # Lifecycle hooks
│       │   ├── memory.rs       # Session memory
│       │   ├── model.rs        # Model profile management
│       │   ├── pref.rs         # User preferences
│       │   ├── session.rs      # Session create/resume/list
│       │   ├── state.rs        # Task state machine
│       │   ├── validate.rs     # Plan validation
│       │   └── verify.rs       # Installation verification
│       └── main.rs
├── templates/
│   ├── commands/hoangsa/       # 14 slash command definitions
│   └── workflows/              # Detailed workflow implementations
│       ├── menu.md             # Design workflow
│       ├── cook.md             # Execution workflow
│       ├── fix.md              # Hotfix workflow
│       ├── audit.md            # Audit workflow
│       ├── research.md         # Research workflow
│       ├── update.md           # Update workflow
│       └── worker-rules.md     # Worker behavior rules
├── bin/
│   └── install                 # Node.js installer script
├── package.json
└── .hoangsa/                   # Project-local config and sessions
    ├── config.json
    └── sessions/               # Session artifacts (plan.json, specs, logs)
```

### Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| CLI | Rust | Session management, DAG traversal, state machine, validation |
| Installer | Node.js | Package distribution, slash command registration |
| Code Intelligence | GitNexus MCP | Call graph, impact analysis, safe rename |
| AI Runtime | Claude Code | Orchestrator + worker execution |

### How to Contribute

1. Fork the repository at https://github.com/pirumu/hoangsa
2. Run `npm run build` to compile the Rust CLI (`cargo build --release` inside `cli/`)
3. Run `npm test` to verify the installation
4. Slash command definitions live in `templates/commands/hoangsa/` — each is a Markdown file with YAML frontmatter
5. Workflow logic lives in `templates/workflows/` — plain Markdown instructions for the AI

---

## Supported Integrations

### Task Managers

- ClickUp
- Asana

### Code Intelligence

- GitNexus MCP (call graphs, impact analysis, execution flow tracing, safe rename)

### Language & Framework Support

HOANGSA is language-agnostic. The worker-rules system has been tested with:

- JavaScript / TypeScript (React, Next.js, Node.js, Bun)
- Rust
- Python (FastAPI, Django)
- Go
- Java / Kotlin (Spring)

---

## License

[MIT](LICENSE) — Copyright (c) 2026 Zan

---

## Author

**Zan** — [@pirumu](https://github.com/pirumu)

---

[Tiếng Việt](README.vi.md)

---
name: hoangsa:cook
description: Execute the plan — wave-by-wave with fresh context per task. Use when the user says "run", "execute", "cook", "start building", "implement the plan", or has a plan.json ready to go.
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - Bash
  - Task
  - TodoWrite
  - AskUserQuestion
---

<objective>
Execute plan.json wave-by-wave. Each task runs in a fresh context window (context engineering). After each worker completes, a /simplify pass auto-runs to fix code quality issues before proceeding. Includes 3-tier verification: static analysis, behavioral tests ×3, semantic review against spec.

Orchestrator role only — dispatches workers, monitors progress, handles escalation.
</objective>

<execution_context>
@~/.claude/hoangsa/workflows/cook.md
</execution_context>

<process>
Follow the cook workflow from `@~/.claude/hoangsa/workflows/cook.md`.
</process>

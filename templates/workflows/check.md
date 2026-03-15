# Check Workflow

Show the current session's task progress overview with wave structure and budget.

## Step 0: Language enforcement

```bash
LANG_PREF=$("~/.claude/hoangsa/bin/hoangsa-cli" pref get . lang)
```

All user-facing text — status updates, reports, progress displays — **MUST** use the language from `lang` preference (`vi` → Vietnamese, `en` → English, `null` → default English). This applies throughout the **ENTIRE** workflow.

---

## Steps

### 1. Locate active session

```bash
SESSION=$("~/.claude/hoangsa/bin/hoangsa-cli" session latest)
```

If `found: false` → inform the user that no active session was found and stop.

### 2. Read plan.json

Read `$SESSION_DIR/plan.json` to load the task list, statuses, and budget.

If no plan.json exists, determine partial session state as follows:

- If `DESIGN-SPEC.md` exists (but no plan.json) → show session ID, status `designing`, and list available specs in the session directory.
- If only `CONTEXT.md` exists → show session ID, status `researching`, and a brief context summary from CONTEXT.md.
- If neither exists → inform user that the session has no artifacts yet.

### 3. Compute waves and tally statuses

```bash
WAVES=$("~/.claude/hoangsa/bin/hoangsa-cli" dag waves "$SESSION_DIR/plan.json")
echo $WAVES
```

Tally tasks by status:
- `passed` or `completed` → done
- `running` or `in_progress` → running
- `pending` or absent status → pending
- `failed` → failed

### 4. Display overview

Print a summary using bilingual labels selected by `$LANG_PREF`. Use the appropriate column below:

| Field | vi | en |
|-------|----|----|
| Session | Phiên | Session |
| Status | Trạng thái | Status |
| Stack | Ngôn ngữ | Stack |
| Budget | Ngân sách | Budget |
| Progress | Tiến độ | Progress |
| Waves | Đợt | Waves |
| Next steps | Bước tiếp | Next steps |

Format:

```
Session / Phiên: <session-id>
Status / Trạng thái:  <overall status>
Stack / Ngôn ngữ:   <language from plan>
Budget / Ngân sách:  <used>k / <total>k tokens (<percent>%)

──────────────────────────────────────────
Wave 1:
  ✅ T-01  <task name>          [passed]   [low,  10k]
  ✅ T-02  <task name>          [passed]   [low,   8k]

Wave 2:
  🔄 T-03  <task name>          [running]  [med,  25k]  ← T-01
  ⬜ T-04  <task name>          [pending]  [med,  20k]  ← T-02

Wave 3:
  ⬜ T-05  <task name>          [pending]  [med,  20k]  ← T-03, T-04
──────────────────────────────────────────

Progress: 2/5 tasks  |  Waves: 1/3 complete

Next steps:
  - /hoangsa:cook   — continue execution
  - /hoangsa:taste  — run acceptance tests
  - /hoangsa:plate  — commit completed work
```

Use only the labels matching `$LANG_PREF` (not both side-by-side as shown above — the table is a reference for which label to use).

Status icons:
- `✅` — passed / completed
- `🔄` — running / in_progress
- `⬜` — pending
- `❌` — failed

Overall status is derived from the task statuses:
- All done → `done`
- Any failed → `failed`
- Any running → `cooking`
- Otherwise → `planning`

### 5. Show available specs (if present)

List which artifacts exist in the session directory. Use `$LANG_PREF` to select the section header (`vi`: "Tài liệu", `en`: "Artifacts"):

```
Artifacts:
  ✅ CONTEXT.md
  ✅ RESEARCH.md
  ✅ DESIGN-SPEC.md
  ✅ TEST-SPEC.md
  ✅ plan.json
  ⬜ project-memory.json
```

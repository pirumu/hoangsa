🇬🇧 [English](README.md)

# HOANGSA

> Hệ thống context engineering 3 giai đoạn (menu → prepare → cook) gọn nhẹ dành cho Claude Code.

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![npm version](https://img.shields.io/npm/v/hoangsa-cc.svg)
![Claude Code](https://img.shields.io/badge/Claude_Code-compatible-blueviolet.svg)
![Built with Rust](https://img.shields.io/badge/Built_with-Rust-orange.svg)
![Node.js](https://img.shields.io/badge/Node.js-18+-green.svg)

---

## HOANGSA là gì?

HOANGSA là hệ thống context engineering dành cho [Claude Code](https://docs.anthropic.com/en/docs/claude-code). Nó giải quyết một vấn đề căn bản: **chất lượng output của Claude giảm dần khi context window bị lấp đầy.**

Giải pháp mang tính cấu trúc. HOANGSA chia công việc thành các task riêng biệt. Mỗi task chạy trong một context window mới với chỉ những file thực sự cần thiết. Kết quả là output nhất quán, chất lượng cao trên các dự án có quy mô tùy ý.

Pipeline cốt lõi gồm ba giai đoạn:

| Giai đoạn | Lệnh | Kết quả |
|-----------|------|---------|
| Thiết kế | `/hoangsa:menu` | DESIGN-SPEC + TEST-SPEC |
| Lập kế hoạch | `/hoangsa:prepare` | DAG task thực thi được (`plan.json`) |
| Thực thi | `/hoangsa:cook` | Code hoàn chỉnh, từng wave một |

Orchestrator không bao giờ viết code. Nó dispatch các worker, mỗi worker có context giới hạn, và tổng hợp kết quả lại.

---

## Tính năng

**Context Engineering** — Mỗi worker task chạy trong context window mới. `context_pointers` trong plan chỉ định chính xác file nào cần đọc — không thừa, không thiếu. Đây là giá trị cốt lõi.

**Phát triển dựa trên Spec** — Mỗi tính năng bắt đầu với DESIGN-SPEC (requirements, interfaces, acceptance criteria) và TEST-SPEC (test cases, coverage targets). Các worker implement theo spec, không phải theo hướng dẫn mơ hồ.

**Thực thi theo DAG** — Các task được tổ chức dưới dạng đồ thị có hướng không chu trình với giải quyết phụ thuộc. Các task độc lập thực thi song song theo wave, các task phụ thuộc thực thi tuần tự. Không có serialization không cần thiết.

**Truy vết Bug Xuyên Tầng** — `/hoangsa:fix` spawn một research agent truy vết bug qua các ranh giới FE/BE/API/DB. Một bug frontend có thể xuất phát từ một API contract phía backend — HOANGSA tìm đúng nguyên nhân gốc rễ trước khi động vào bất kỳ dòng code nào.

**Audit Codebase 8 Chiều** — `/hoangsa:audit` quét code smells, lỗ hổng bảo mật, bottleneck hiệu năng, tech debt, khoảng trống test coverage, rủi ro dependency, vi phạm kiến trúc, và thiếu hụt tài liệu.

**Tích hợp Task Manager** — Dán link task (ClickUp, Asana) bất kỳ đâu trong workflow. HOANGSA kéo thông tin task làm context bổ sung và đồng bộ kết quả ngược lại (cập nhật trạng thái, bình luận, báo cáo) sau khi công việc hoàn thành.

**GitNexus Code Intelligence** — Phân tích call graph tích hợp sẵn. Chạy impact analysis trước mỗi lần sửa, đổi tên an toàn trên toàn bộ codebase, và truy vết toàn bộ execution flow từ entry point đến hàm lá.

**Chọn Model Đa Profile** — Chuyển đổi giữa các profile quality, balanced, và budget để phù hợp với yêu cầu task và ràng buộc chi phí.

---

## Bắt đầu nhanh

```bash
npx hoangsa-cc          # Cài HOANGSA vào môi trường Claude Code
/hoangsa:init           # Khởi tạo project — phát hiện codebase, cài đặt preferences
/hoangsa:menu           # Thiết kế task đầu tiên của bạn
```

Sau khi `/hoangsa:menu` hoàn thành, tiếp tục với `/hoangsa:prepare` để tạo plan, rồi `/hoangsa:cook` để thực thi.

---

## Cài đặt

Yêu cầu: **Node.js 18+** và **[Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code)**

```bash
# Tương tác — hỏi cài global hay local
npx hoangsa-cc

# Cài vào ~/.claude/ — dùng được ở mọi project
npx hoangsa-cc --global

# Cài vào .claude/ — chỉ project này
npx hoangsa-cc --local

# Gỡ HOANGSA
npx hoangsa-cc --uninstall

# Cài vào thư mục config tùy chỉnh
npx hoangsa-cc --config-dir <path>
```

| Flag | Viết tắt | Mô tả |
|------|----------|-------|
| `--global` | `-g` | Cài vào `~/.claude/` (tất cả projects) |
| `--local` | `-l` | Cài vào `.claude/` (chỉ project này) |
| `--uninstall` | `-u` | Gỡ HOANGSA |
| `--config-dir` | | Sử dụng đường dẫn thư mục config tùy chỉnh |

---

## Quy trình

```
ý tưởng  →  /menu      Thiết kế  →  DESIGN-SPEC + TEST-SPEC
         →  /prepare   Lập kế hoạch  →  DAG task thực thi được (plan.json)
         →  /cook      Thực thi  →  Từng wave, context mới cho mỗi task
         →  /taste     Kiểm tra  →  Acceptance tests từng task
         →  /plate     Commit    →  Conventional commit message
         →  /serve     Đồng bộ   →  Sync hai chiều với task manager
```

**Thiết kế (`/menu`)** — Phỏng vấn người dùng về requirements. Tạo ra DESIGN-SPEC có cấu trúc với interfaces và acceptance criteria, cùng TEST-SPEC với test cases và coverage targets.

**Lập kế hoạch (`/prepare`)** — Phân tích specs và tạo `plan.json`: một DAG các task, mỗi task được gán worker, danh sách file giới hạn (`context_pointers`), và các cạnh dependency tường minh.

**Thực thi (`/cook`)** — Đi qua DAG từng wave. Dispatch mỗi worker cùng context của nó. Các task độc lập trong cùng wave có thể chạy song song. Tổng hợp kết quả trước khi tiến sang wave tiếp theo.

**Kiểm tra (`/taste`)** — Chạy các acceptance tests được định nghĩa trong TEST-SPEC. Báo cáo pass/fail từng task. Chặn pipeline khi có lỗi.

**Commit (`/plate`)** — Stage các thay đổi và tạo conventional commit message từ công việc đã hoàn thành.

**Đồng bộ (`/serve`)** — Đẩy cập nhật trạng thái, bình luận, và artifacts về task manager được kết nối.

---

## Lệnh

### Quy trình cốt lõi

| Lệnh | Mô tả |
|------|-------|
| `/hoangsa:menu` | Thiết kế — từ ý tưởng đến DESIGN-SPEC + TEST-SPEC |
| `/hoangsa:prepare` | Lập kế hoạch — chuyển specs thành DAG task thực thi được |
| `/hoangsa:cook` | Thực thi — từng wave với context mới cho mỗi task |
| `/hoangsa:taste` | Kiểm tra — chạy acceptance tests từng task |
| `/hoangsa:plate` | Commit — tạo và áp dụng conventional commit message |
| `/hoangsa:serve` | Đồng bộ — sync hai chiều với task manager được kết nối |

### Chuyên biệt

| Lệnh | Mô tả |
|------|-------|
| `/hoangsa:fix` | Hotfix — truy vết root cause xuyên tầng + fix gọn có mục tiêu |
| `/hoangsa:audit` | Audit — quét codebase 8 chiều (bảo mật, tech debt, coverage, v.v.) |
| `/hoangsa:research` | Research — phân tích codebase kết hợp nghiên cứu bên ngoài |

### Tiện ích

| Lệnh | Mô tả |
|------|-------|
| `/hoangsa:init` | Khởi tạo — phát hiện codebase, cấu hình preferences, thiết lập lần đầu |
| `/hoangsa:check` | Trạng thái — hiển thị tiến độ session hiện tại và các task đang chờ |
| `/hoangsa:index` | Index — xây dựng lại đồ thị code intelligence GitNexus |
| `/hoangsa:update` | Cập nhật — nâng cấp HOANGSA lên phiên bản mới nhất |
| `/hoangsa:help` | Trợ giúp — hiển thị tất cả lệnh có sẵn |

---

## Cấu hình

HOANGSA lưu cấu hình project trong `.hoangsa/config.json`.

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

| Khóa | Giá trị | Mô tả |
|------|---------|-------|
| `lang` | `en`, `vi` | Ngôn ngữ cho output của orchestrator |
| `spec_lang` | `en`, `vi` | Ngôn ngữ cho các spec được tạo ra |
| `tech_stack` | array | Tech stack của project (dùng để tinh chỉnh hướng dẫn worker) |
| `review_style` | `strict`, `balanced`, `light` | Mức độ kỹ lưỡng khi review code |

### Model Profiles

Chọn profile để kiểm soát model được dùng ở mỗi vai trò:

| Profile | Worker | Designer | Reviewer |
|---------|--------|----------|----------|
| `quality` | claude-opus | claude-opus | claude-opus |
| `balanced` | claude-sonnet | claude-opus | claude-sonnet |
| `budget` | claude-haiku | claude-sonnet | claude-haiku |

Chuyển đổi profile bằng `/hoangsa:init` hoặc sửa `model_profile` trong `config.json`.

### Tích hợp Task Manager

| Provider | Cách kết nối |
|----------|-------------|
| ClickUp | Dán URL ClickUp task |
| Asana | Dán URL Asana task |

HOANGSA kéo thông tin task làm context bổ sung và ghi kết quả về khi chạy `/hoangsa:serve`.

---

## Kiến trúc

### Cấu trúc Project

```
hoangsa/
├── cli/                        # Rust CLI (hoangsa-cli)
│   └── src/
│       ├── cmd/                # 13 command modules
│       │   ├── config.rs       # Đọc/ghi config
│       │   ├── context.rs      # Phân giải context pointer
│       │   ├── dag.rs          # Duyệt DAG và lên lịch wave
│       │   ├── hook.rs         # Lifecycle hooks
│       │   ├── memory.rs       # Bộ nhớ session
│       │   ├── model.rs        # Quản lý model profile
│       │   ├── pref.rs         # Preferences người dùng
│       │   ├── session.rs      # Tạo/tiếp tục/liệt kê session
│       │   ├── state.rs        # State machine task
│       │   ├── validate.rs     # Kiểm tra tính hợp lệ của plan
│       │   └── verify.rs       # Xác minh cài đặt
│       └── main.rs
├── templates/
│   ├── commands/hoangsa/       # 14 định nghĩa slash command
│   └── workflows/              # Triển khai workflow chi tiết
│       ├── menu.md             # Workflow thiết kế
│       ├── cook.md             # Workflow thực thi
│       ├── fix.md              # Workflow hotfix
│       ├── audit.md            # Workflow audit
│       ├── research.md         # Workflow research
│       ├── update.md           # Workflow cập nhật
│       └── worker-rules.md     # Quy tắc hành vi worker
├── bin/
│   └── install                 # Script installer Node.js
├── package.json
└── .hoangsa/                   # Config và sessions cục bộ của project
    ├── config.json
    └── sessions/               # Artifacts session (plan.json, specs, logs)
```

### Tech Stack

| Tầng | Công nghệ | Mục đích |
|------|-----------|---------|
| CLI | Rust | Quản lý session, duyệt DAG, state machine, validation |
| Installer | Node.js | Phân phối package, đăng ký slash command |
| Code Intelligence | GitNexus MCP | Call graph, impact analysis, đổi tên an toàn |
| AI Runtime | Claude Code | Thực thi orchestrator + worker |

### Cách đóng góp

1. Fork repository tại https://github.com/pirumu/hoangsa
2. Chạy `npm run build` để biên dịch Rust CLI (`cargo build --release` bên trong `cli/`)
3. Chạy `npm test` để xác minh cài đặt
4. Định nghĩa slash command nằm trong `templates/commands/hoangsa/` — mỗi file là Markdown với YAML frontmatter
5. Logic workflow nằm trong `templates/workflows/` — hướng dẫn Markdown thuần cho AI

---

## Tích hợp hỗ trợ

### Task Managers

- ClickUp
- Asana

### Code Intelligence

- GitNexus MCP (call graphs, impact analysis, truy vết execution flow, đổi tên an toàn)

### Hỗ trợ Ngôn ngữ & Framework

HOANGSA không phụ thuộc vào ngôn ngữ cụ thể. Hệ thống worker-rules đã được kiểm thử với:

- JavaScript / TypeScript (React, Next.js, Node.js, Bun)
- Rust
- Python (FastAPI, Django)
- Go
- Java / Kotlin (Spring)

---

## Giấy phép

[MIT](LICENSE) — Copyright (c) 2026 Zan

---

## Tác giả

**Zan** — [@pirumu](https://github.com/pirumu)

---

[English](README.md)

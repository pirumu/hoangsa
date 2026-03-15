use crate::cmd::dag::{detect_cycles, detect_dangling};
use crate::helpers::{is_absolute, out, parse_frontmatter, read_file, read_json};
use serde_json::{Value, json};
use std::path::Path;

/// `validate plan <path>`
pub fn cmd_plan(file_path: &str) {
    if !Path::new(file_path).exists() {
        out(&json!({ "valid": false, "errors": [format!("Plan file not found: {}", file_path)] }));
        return;
    }
    let plan = read_json(file_path);
    if plan.get("error").is_some() {
        out(&json!({ "valid": false, "errors": [plan["error"]] }));
        return;
    }

    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    for f in &["name", "workspace_dir", "budget_tokens", "tasks"] {
        if plan.get(f).is_none() {
            errors.push(format!("Missing field: {f}"));
        }
    }

    let tasks = plan.get("tasks").and_then(|v| v.as_array());
    match tasks {
        Some(arr) if arr.is_empty() => {
            errors.push("tasks must be a non-empty array".to_string());
        }
        None => {
            if plan.get("tasks").is_some() {
                errors.push("tasks must be a non-empty array".to_string());
            }
        }
        _ => {}
    }

    if let Some(wd) = plan.get("workspace_dir").and_then(|v| v.as_str()) {
        if !is_absolute(wd) {
            errors.push("workspace_dir must be an absolute path".to_string());
        }
    }

    if let Some(task_arr) = tasks {
        for t in task_arr {
            let tid = t.get("id").and_then(|v| v.as_str()).unwrap_or("?");
            let required = [
                "id",
                "name",
                "complexity",
                "budget_tokens",
                "files",
                "depends_on",
                "context_pointers",
                "acceptance",
            ];
            for f in &required {
                if t.get(f).is_none() {
                    errors.push(format!("Task {tid}: missing {f}"));
                }
            }
            if let Some(complexity) = t.get("complexity").and_then(|v| v.as_str()) {
                if !["low", "medium", "high"].contains(&complexity) {
                    errors.push(format!("Task {tid}: complexity must be low|medium|high"));
                }
            }
            if let Some(budget) = t.get("budget_tokens").and_then(|v| v.as_u64()) {
                if budget > 45000 {
                    warnings.push(format!("Task {tid}: budget {budget} exceeds 45k limit"));
                }
            }
            match t.get("files").and_then(|v| v.as_array()) {
                Some(files) if files.is_empty() => {
                    errors.push(format!("Task {tid}: files must be non-empty array"));
                }
                Some(files) => {
                    for f in files {
                        if let Some(fp) = f.as_str() {
                            if !is_absolute(fp) {
                                errors
                                    .push(format!("Task {tid}: file path not absolute: {fp}"));
                            }
                        }
                    }
                }
                None => {
                    errors.push(format!("Task {tid}: files must be non-empty array"));
                }
            }
            if let Some(acceptance) = t.get("acceptance").and_then(|v| v.as_str()) {
                let trimmed = acceptance.trim();
                if !trimmed.is_empty() {
                    if let Some(first_char) = trimmed.chars().next() {
                        if !first_char.is_ascii_lowercase() {
                            warnings.push(format!(
                                "Task {tid}: acceptance may not be a runnable command"
                            ));
                        }
                    }
                }
            }
        }
    }

    // DAG checks
    if let Some(task_arr) = tasks {
        let cycles = detect_cycles(task_arr);
        let dangling = detect_dangling(task_arr);
        for c in cycles {
            errors.push(format!("Cycle: {c}"));
        }
        errors.extend(dangling);
    }

    // Budget sanity
    if let (Some(task_arr), Some(total_budget)) =
        (tasks, plan.get("budget_tokens").and_then(|v| v.as_f64()))
    {
        if total_budget > 0.0 {
            let sum: f64 = task_arr
                .iter()
                .filter_map(|t| t.get("budget_tokens").and_then(|v| v.as_f64()))
                .sum();
            if ((sum - total_budget) / total_budget).abs() > 0.1 {
                warnings.push(format!(
                    "Budget mismatch: declared {}, tasks sum to {}",
                    total_budget as u64, sum as u64
                ));
            }
        }
    }

    let task_count = tasks.map(|a| a.len()).unwrap_or(0);
    out(&json!({
        "valid": errors.is_empty(),
        "errors": errors,
        "warnings": warnings,
        "task_count": task_count,
    }));
}

/// `validate spec <path>`
pub fn cmd_spec(file_path: &str) {
    let content = match read_file(file_path) {
        Some(c) => c,
        None => {
            out(&json!({ "valid": false, "errors": ["File not found"] }));
            return;
        }
    };

    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let fm = parse_frontmatter(&content);

    match &fm {
        None => {
            errors.push("Missing YAML frontmatter (--- delimiters)".to_string());
        }
        Some(map) => {
            for f in &["spec_version", "project", "component", "language", "status"] {
                if !map.contains_key(*f) {
                    errors.push(format!("Frontmatter missing: {f}"));
                }
            }
        }
    }

    if !content.contains("## Types") {
        warnings.push("Missing ## Types section".to_string());
    }
    if !content.contains("## Interfaces") {
        warnings.push("Missing ## Interfaces section".to_string());
    }
    if !content.contains("## Implementations") {
        errors.push("Missing ## Implementations section".to_string());
    }
    if !content.contains("## Acceptance") {
        errors.push("Missing ## Acceptance Criteria section".to_string());
    }

    let code_block_count = content.matches("```").count() / 2;
    if code_block_count < 2 {
        warnings.push("Expected code blocks in Types and Interfaces sections".to_string());
    }

    let component = fm
        .as_ref()
        .and_then(|m| m.get("component"))
        .map(|s| Value::String(s.clone()))
        .unwrap_or(Value::Null);

    out(&json!({
        "valid": errors.is_empty(),
        "errors": errors,
        "warnings": warnings,
        "component": component,
    }));
}

/// `validate tests <path>`
pub fn cmd_tests(file_path: &str) {
    let content = match read_file(file_path) {
        Some(c) => c,
        None => {
            out(&json!({ "valid": false, "errors": ["File not found"] }));
            return;
        }
    };

    let mut errors: Vec<String> = Vec::new();
    let warnings: Vec<String> = Vec::new();
    let fm = parse_frontmatter(&content);

    match &fm {
        None => {
            errors.push("Missing YAML frontmatter".to_string());
        }
        Some(map) => {
            for f in &["tests_version", "spec_ref", "component"] {
                if !map.contains_key(*f) {
                    errors.push(format!("Frontmatter missing: {f}"));
                }
            }
        }
    }

    let has_unit = content.contains("## Unit Tests");
    let has_integration = content.contains("## Integration Tests");
    if !has_unit && !has_integration {
        errors.push("Must have at least one of: ## Unit Tests, ## Integration Tests".to_string());
    }

    let component = fm
        .as_ref()
        .and_then(|m| m.get("component"))
        .map(|s| Value::String(s.clone()))
        .unwrap_or(Value::Null);

    out(&json!({
        "valid": errors.is_empty(),
        "errors": errors,
        "warnings": warnings,
        "component": component,
    }));
}

use crate::helpers::{out, read_json};
use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;
use time::OffsetDateTime;
use time::macros::format_description;

fn now_iso() -> String {
    let now = OffsetDateTime::now_utc();
    now.format(format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
    ))
    .unwrap_or_default()
}

/// `memory init <projectDir>`
pub fn cmd_init(project_dir: Option<&str>) {
    let project_dir = match project_dir {
        Some(d) => d,
        None => {
            out(&json!({ "error": "projectDir is required" }));
            return;
        }
    };

    let memory_dir = Path::new(project_dir).join(".hoangsa");
    let memory_file = memory_dir.join("project-memory.json");

    if memory_file.exists() {
        out(&json!({
            "error": "project-memory.json already exists",
            "path": memory_file.to_string_lossy(),
        }));
        return;
    }

    // Detect project info
    let mut project_name = Path::new(project_dir)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    let mut stack = "unknown".to_string();
    let mut test_framework = "unknown".to_string();

    let pkg_path = Path::new(project_dir).join("package.json");
    let cargo_path = Path::new(project_dir).join("Cargo.toml");
    let requirements_path = Path::new(project_dir).join("requirements.txt");
    let setup_py_path = Path::new(project_dir).join("setup.py");
    let pyproject_path = Path::new(project_dir).join("pyproject.toml");
    let go_mod_path = Path::new(project_dir).join("go.mod");

    if pkg_path.exists() {
        if let Ok(content) = fs::read_to_string(&pkg_path) {
            if let Ok(pkg) = serde_json::from_str::<Value>(&content) {
                if let Some(name) = pkg.get("name").and_then(|v| v.as_str()) {
                    project_name = name.to_string();
                }
                stack = "node".to_string();
                // Detect test framework from dependencies
                let deps = pkg.get("dependencies").and_then(|v| v.as_object());
                let dev_deps = pkg.get("devDependencies").and_then(|v| v.as_object());
                let mut all_deps: Map<String, Value> = Map::new();
                if let Some(d) = deps {
                    all_deps.extend(d.clone());
                }
                if let Some(d) = dev_deps {
                    all_deps.extend(d.clone());
                }

                if all_deps.contains_key("jest") {
                    test_framework = "jest".to_string();
                } else if all_deps.contains_key("mocha") {
                    test_framework = "mocha".to_string();
                } else if all_deps.contains_key("vitest") {
                    test_framework = "vitest".to_string();
                } else {
                    test_framework = "node:test".to_string();
                }
            }
        }
    } else if cargo_path.exists() {
        stack = "rust".to_string();
        test_framework = "cargo test".to_string();
    } else if requirements_path.exists() || setup_py_path.exists() || pyproject_path.exists() {
        stack = "python".to_string();
        test_framework = "pytest".to_string();
    } else if go_mod_path.exists() {
        stack = "go".to_string();
        test_framework = "go test".to_string();
    }

    let async_pattern = if stack == "node" {
        "async/await"
    } else {
        "sync"
    };

    let now = now_iso();
    let memory = json!({
        "version": 1,
        "project": project_name,
        "stack": stack,
        "conventions": {
            "naming": "camelCase functions, snake_case JSON",
            "error_handling": "try-catch + { error } return",
            "async_pattern": async_pattern,
            "test_framework": test_framework,
        },
        "key_types": [],
        "key_interfaces": [],
        "updated_at": now,
    });

    if let Err(e) = fs::create_dir_all(&memory_dir) {
        out(&json!({ "success": false, "error": e.to_string() }));
        return;
    }
    match fs::write(&memory_file, serde_json::to_string_pretty(&memory).unwrap()) {
        Ok(_) => out(&json!({
            "success": true,
            "path": memory_file.to_string_lossy(),
            "memory": memory,
        })),
        Err(e) => out(&json!({ "success": false, "error": e.to_string() })),
    }
}

/// `memory get <projectDir>`
pub fn cmd_get(project_dir: Option<&str>) {
    let project_dir = match project_dir {
        Some(d) => d,
        None => {
            out(&json!({ "error": "projectDir is required" }));
            return;
        }
    };
    let memory_file = Path::new(project_dir)
        .join(".hoangsa")
        .join("project-memory.json");
    if !memory_file.exists() {
        out(&json!({ "error": format!("project-memory.json not found at {}. Run `memory init` first.", memory_file.display()) }));
        return;
    }
    let memory = read_json(memory_file.to_str().unwrap_or(""));
    if memory.get("error").is_some() {
        out(&json!({ "error": memory["error"] }));
        return;
    }
    out(&memory);
}

/// `memory update <projectDir> <jsonPatch>`
pub fn cmd_update(project_dir: Option<&str>, json_patch: Option<&str>) {
    let project_dir = match project_dir {
        Some(d) => d,
        None => {
            out(&json!({ "error": "projectDir is required" }));
            return;
        }
    };
    let json_patch = match json_patch {
        Some(p) => p,
        None => {
            out(&json!({ "error": "jsonPatch is required" }));
            return;
        }
    };

    let memory_file = Path::new(project_dir)
        .join(".hoangsa")
        .join("project-memory.json");
    if !memory_file.exists() {
        out(&json!({ "error": format!("project-memory.json not found at {}. Run `memory init` first.", memory_file.display()) }));
        return;
    }
    let memory = read_json(memory_file.to_str().unwrap_or(""));
    if memory.get("error").is_some() {
        out(&json!({ "error": memory["error"] }));
        return;
    }

    let patch: Value = match serde_json::from_str(json_patch) {
        Ok(v) => v,
        Err(e) => {
            out(&json!({ "error": format!("Invalid JSON patch: {}", e) }));
            return;
        }
    };

    let mut updated = memory.as_object().cloned().unwrap_or_default();
    if let Some(patch_obj) = patch.as_object() {
        for (k, v) in patch_obj {
            updated.insert(k.clone(), v.clone());
        }
    }
    updated.insert("updated_at".to_string(), json!(now_iso()));

    // Deep merge conventions if patch includes it
    if let (Some(patch_conv), Some(mem_conv)) = (
        patch.get("conventions").and_then(|v| v.as_object()),
        memory.get("conventions").and_then(|v| v.as_object()),
    ) {
        let mut merged: Map<String, Value> = mem_conv.clone();
        for (k, v) in patch_conv {
            merged.insert(k.clone(), v.clone());
        }
        updated.insert("conventions".to_string(), Value::Object(merged));
    }

    let updated_val = Value::Object(updated);
    match fs::write(
        &memory_file,
        serde_json::to_string_pretty(&updated_val).unwrap(),
    ) {
        Ok(_) => out(&json!({ "success": true, "memory": updated_val })),
        Err(e) => out(&json!({ "success": false, "error": e.to_string() })),
    }
}

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn find_active_plan_path() -> Option<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()?;
    let brain_root = Path::new(&home).join(".gemini/antigravity-ide/brain");
    if !brain_root.is_dir() {
        return None;
    }

    let mut best_path = None;
    let mut best_mtime = None;

    if let Ok(entries) = fs::read_dir(brain_root) {
        for entry in entries.flatten() {
            let path = entry.path().join("implementation_plan.md");
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        if best_mtime.is_none() || modified > best_mtime.unwrap() {
                            best_mtime = Some(modified);
                            best_path = Some(path);
                        }
                    }
                }
            }
        }
    }

    best_path
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking active implementation plan...");
    let plan_path = match find_active_plan_path() {
        Some(path) => path,
        None => {
            eprintln!("Error: No active implementation_plan.md found under brain directory.");
            std::process::exit(1);
        }
    };

    println!("Found plan at: {}", plan_path.display());
    let plan_content = fs::read_to_string(&plan_path)?;

    // Verify ANTIGRAVITY_LS_ADDRESS is set
    if std::env::var("ANTIGRAVITY_LS_ADDRESS").is_err() {
        eprintln!("Error: ANTIGRAVITY_LS_ADDRESS is not set.");
        eprintln!("Please ensure this command is run from a terminal spawned inside the Antigravity IDE,");
        eprintln!("or set the environment variable manually (e.g., export ANTIGRAVITY_LS_ADDRESS=localhost:<port>).");
        std::process::exit(1);
    }

    // --- CUSTOMIZE THIS SYSTEM PROMPT TO YOUR NEW REPO ---
    let system_instruction = "You are an expert systems engineer reviewing an implementation plan for Paradox Plus (a Bevy-based multiplayer game).\n\
Your task is to analyze the implementation plan for alignment with these architectural rules:\n\
1. 300-Line Limit: No individual Rust source file (excluding test suites or benchmarks) may exceed 300 lines of code. Granular modularization is required.\n\
2. Pure Rust ECS (No DOM): Absolutely never use HTML, CSS, JavaScript, WebViews, or DOM elements. All UI must use native Bevy UI (bevy_ui, Taffy, WGSL shaders).\n\
3. Authoritative Server Validation: Gameplay state mutations, card draws, and movement resolutions must be evaluated on the authoritative Server. The Client only renders interpolated state and sends actions.\n\
4. Type-Safe Serialization: All network payloads must be serialized/deserialized using Postcard and compile-time verified structs/enums shared in the protocol crate.\n\
5. Logic & Performance: Audit for logic bugs, memory issues, or performance constraints.\n\
6. ADR Verification: Ensure that the plan contains a dedicated checklist task to evaluate if any system-level design pivots occurred and document them in an Architecture Decision Record (ADR) under doc/adr/.\n\n\
Respond with a constructive markdown critique. Include specific warnings, proposed patches or code snippets, and list any outstanding open questions.\n\n\
IMPORTANT: You MUST NOT execute any commands, use any tools, or read/write files in this workspace. Do not output any agent planning thoughts, introduction, or summary. Output ONLY the markdown critique.";

    let prompt = format!(
        "{}\n\nHere is the implementation plan:\n\n{}",
        system_instruction, plan_content
    );

    println!("Sending implementation plan to Antigravity CLI (agy) for critique...");

    // Try executing "agy" first from PATH
    let mut child = match Command::new("agy")
        .arg("--sandbox")
        .arg("--print")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .unwrap_or_default();
            let bin_name = if cfg!(windows) { "agy.exe" } else { "agy" };
            let local_agy = Path::new(&home).join(".local").join("bin").join(bin_name);
            println!(
                "'agy' not found in PATH, falling back to: {}",
                local_agy.display()
            );
            Command::new(&local_agy)
                .arg("--sandbox")
                .arg("--print")
                .arg("-")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|err| {
                    format!(
                        "Failed to execute `agy` binary: {}. Also tried local fallback at {}: {}",
                        e,
                        local_agy.display(),
                        err
                    )
                })?
        }
        Err(e) => return Err(e.into()),
    };

    // Pipe the prompt to stdin and close stdin to allow command to execute
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(prompt.as_bytes())?;
        stdin.flush()?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        eprintln!("Error: `agy` exited with status {}.", output.status);
        if !stdout_str.is_empty() {
            eprintln!("Stdout:\n{}", stdout_str);
        }
        if !stderr_str.is_empty() {
            eprintln!("Stderr:\n{}", stderr_str);
        }
        std::process::exit(1);
    }

    let critique_text = String::from_utf8_lossy(&output.stdout).into_owned();

    let dest_dir = plan_path
        .parent()
        .ok_or("Could not resolve parent directory of plan")?;
    let dest_path = dest_dir.join("implementation_plan_critique.md");
    let temp_path = dest_dir.join("implementation_plan_critique.md.tmp");

    // Atomic write/rename
    fs::write(&temp_path, &critique_text)?;
    fs::rename(&temp_path, &dest_path)?;

    println!(
        "\nCritique successfully written to: {}\n",
        dest_path.display()
    );
    println!("--- CRITIQUE SUMMARY ---");
    let lines: Vec<&str> = critique_text.lines().collect();
    for line in lines.iter().take(15) {
        println!("{}", line);
    }
    if lines.len() > 15 {
        println!("...");
    }
    println!("------------------------");

    Ok(())
}

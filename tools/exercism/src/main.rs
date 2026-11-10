use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "exercism", about = "Rust Exercism 顺序练习工具")]
struct Cli {
    #[command(subcommand)]
    command: Option<CommandLine>,
}

#[derive(Subcommand)]
enum CommandLine {
    Start,
    Concept,
    List,
}

#[derive(Deserialize)]
struct TrackConfig {
    exercises: ExerciseGroups,
}

#[derive(Deserialize)]
struct ExerciseGroups {
    practice: Vec<Exercise>,
    concept: Vec<Exercise>,
}

#[derive(Deserialize)]
struct Exercise {
    slug: String,
    name: String,
    difficulty: u8,
    status: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct Progress {
    completed: BTreeSet<String>,
}

struct ExerciseInfo {
    exercise: Exercise,
    package_name: String,
    directory: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(CommandLine::Start);
    let root = project_root()?;

    match command {
        CommandLine::Start => start(&root, &load_exercises(&root, "practice")?)?,
        CommandLine::Concept => start(&root, &load_exercises(&root, "concept")?)?,
        CommandLine::List => list(&root, &load_exercises(&root, "practice")?)?,
    }

    Ok(())
}

fn project_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_directory
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or("无法定位项目根目录")?;
    Ok(root)
}

fn progress_path(root: &Path) -> PathBuf {
    root.join("exercises/.ex-progress.json")
}

fn load_progress(root: &Path) -> Progress {
    let path = progress_path(root);
    if !path.exists() {
        return Progress::default();
    }
    fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

fn save_progress(root: &Path, progress: &Progress) -> Result<(), Box<dyn std::error::Error>> {
    let path = progress_path(root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_string_pretty(progress)?)?;
    Ok(())
}

fn mark_completed(
    root: &Path,
    package_name: &str,
    passed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut progress = load_progress(root);
    if passed {
        progress.completed.insert(package_name.to_string());
    } else {
        progress.completed.remove(package_name);
    }
    save_progress(root, &progress)
}

fn load_exercises(
    root: &Path,
    track: &str,
) -> Result<Vec<ExerciseInfo>, Box<dyn std::error::Error>> {
    let config_path = root.join("exercises/track-config.json");
    let config: TrackConfig = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    let configured_exercises = match track {
        "practice" => config.exercises.practice,
        "concept" => config.exercises.concept,
        _ => return Err("不支持的练习类型".into()),
    };
    let mut exercises = configured_exercises
        .into_iter()
        .filter_map(|exercise| {
            let directory = root.join("exercises").join(track).join(&exercise.slug);
            let package_name = package_name(&directory).ok()?;
            Some(ExerciseInfo {
                exercise,
                package_name,
                directory,
            })
        })
        .collect::<Vec<_>>();

    exercises.sort_by_key(|exercise| exercise.exercise.difficulty);
    Ok(exercises)
}

fn package_name(directory: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(directory.join("Cargo.toml"))?;
    let package_section = content
        .split_once("[package]")
        .map(|(_, section)| section)
        .ok_or("题目 Cargo.toml 缺少 package 配置")?;
    let name = package_section
        .lines()
        .find_map(|line| line.trim().strip_prefix("name = "))
        .map(|name| name.trim_matches('"'))
        .ok_or("题目 Cargo.toml 缺少 package name")?;
    Ok(name.to_string())
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}

fn start(root: &Path, exercises: &[ExerciseInfo]) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_index = 0;
    while current_index < exercises.len() {
        let current = &exercises[current_index];
        let (passed_quietly, _) = run_test(root, current, true)?;
        if passed_quietly {
            mark_completed(root, &current.package_name, true)?;
            current_index += 1;
            continue;
        }

        print_current(root, exercises, current_index);
        open_source(current);
        let (passed, summary) = run_test(root, current, false)?;
        mark_completed(root, &current.package_name, passed)?;
        println!("{summary}");
        if passed {
            current_index += 1;
            if current_index < exercises.len() {
                println!("下一题：{}", exercises[current_index].exercise.name);
            }
            continue;
        }

        println!("未通过。改完再运行 `cargo ex start`。");
        break;
    }

    if current_index == exercises.len() {
        println!("全部 {} 道练习已通过。", exercises.len());
    }
    Ok(())
}

fn run_test(
    root: &Path,
    exercise: &ExerciseInfo,
    quiet: bool,
) -> Result<(bool, String), Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .current_dir(root)
        .args(["test", "-p", &exercise.package_name, "--", "--test-threads=1"])
        .output()?;
    let passed = output.status.success();
    if quiet {
        return Ok((passed, String::new()));
    }
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok((passed, summarize_test_output(&combined, passed)))
}

fn summarize_test_output(output: &str, passed: bool) -> String {
    if passed {
        return "✅ 测试通过".to_string();
    }

    let mut lines = Vec::new();
    if let Some(result_line) = output
        .lines()
        .rev()
        .find(|line| line.starts_with("test result:"))
    {
        lines.push(format!("❌ {result_line}"));
    } else {
        lines.push("❌ 测试未通过".to_string());
    }

    let mut failure_names = Vec::new();
    let mut in_failure_list = false;
    for line in output.lines() {
        if line == "failures:" {
            in_failure_list = true;
            continue;
        }
        if !in_failure_list {
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            in_failure_list = false;
            continue;
        }
        if trimmed.starts_with("test result:") {
            break;
        }
        failure_names.push(trimmed.to_string());
    }

    for name in &failure_names {
        lines.push(format!("  - {name}"));
        if let Some(detail) = failure_detail(output, name) {
            lines.push(format!("    {detail}"));
        }
    }

    if failure_names.is_empty() {
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("error[") || trimmed.starts_with("error:") {
                lines.push(format!("  {trimmed}"));
            }
        }
    }

    lines.join("\n")
}

fn failure_detail(output: &str, name: &str) -> Option<String> {
    let marker = format!("---- {name} stdout ----");
    let block: Vec<&str> = output
        .lines()
        .skip_while(|line| line.trim() != marker)
        .skip(1)
        .take_while(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("---- ") && trimmed != "failures:"
        })
        .collect();

    let mut details = Vec::new();
    for trimmed in block.iter().map(|line| line.trim()) {
        if trimmed.starts_with("thread ") || trimmed.starts_with("note:") {
            continue;
        }
        if trimmed.starts_with("assertion `left == right` failed") {
            details.push("assertion failed".to_string());
            continue;
        }
        if trimmed.starts_with("left:") || trimmed.starts_with("right:") {
            details.push(trimmed.to_string());
            continue;
        }
        if !trimmed.is_empty() {
            details.push(trimmed.to_string());
        }
    }

    if details.is_empty() {
        return None;
    }
    Some(details.join(" | "))
}

fn print_current(root: &Path, exercises: &[ExerciseInfo], current_index: usize) {
    let current = &exercises[current_index];
    let total = exercises.len();
    let source = current.directory.join("src/lib.rs");
    let instructions = current.directory.join(".docs/instructions.md");
    println!(
        "进度：{current_index}/{total}（{:.0}%）",
        current_index as f64 / total as f64 * 100.0
    );
    println!(
        "当前：{}（难度 {}）",
        current.exercise.name, current.exercise.difficulty
    );
    println!("源码：{}", relative_path(root, &source));
    if instructions.exists() {
        println!("说明：{}", relative_path(root, &instructions));
    }
}

fn open_source(exercise: &ExerciseInfo) {
    let source = exercise.directory.join("src/lib.rs");
    if !source.exists() {
        return;
    }

    let mut editors = Vec::new();
    if let Ok(editor) = std::env::var("EX_EDITOR") {
        if !editor.trim().is_empty() {
            editors.push(editor);
        }
    }
    editors.extend(["zed".to_string(), "Zed".to_string()]);

    for editor in editors {
        let status = Command::new(&editor).arg(&source).status();
        if status.map(|result| result.success()).unwrap_or(false) {
            println!("已用 {editor} 打开源码。");
            return;
        }
    }
    println!("未能自动打开编辑器，请手动打开源码。");
}

fn list(root: &Path, exercises: &[ExerciseInfo]) -> Result<(), Box<dyn std::error::Error>> {
    let progress = load_progress(root);
    let mut completed = 0usize;
    for (index, exercise) in exercises.iter().enumerate() {
        let done = progress.completed.contains(&exercise.package_name);
        if done {
            completed += 1;
        }
        let mark = if done { "✓" } else { " " };
        let status = exercise.exercise.status.as_deref().unwrap_or("active");
        if status == "active" {
            println!(
                "[{mark}] {:>3}. 难度 {:>2}  {}",
                index + 1,
                exercise.exercise.difficulty,
                exercise.exercise.name
            );
        } else {
            println!(
                "[{mark}] {:>3}. 难度 {:>2}  {} ({status})",
                index + 1,
                exercise.exercise.difficulty,
                exercise.exercise.name
            );
        }
    }
    println!(
        "\n进度：{completed}/{}（{:.0}%）",
        exercises.len(),
        completed as f64 / exercises.len() as f64 * 100.0
    );
    Ok(())
}

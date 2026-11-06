use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    sync::mpsc::channel,
    time::Duration,
};

use clap::{Parser, Subcommand};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;

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
    Watch,
    List,
    Show,
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

struct ExerciseInfo {
    exercise: Exercise,
    package_name: String,
    directory: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(CommandLine::Start);
    let root = project_root()?;
    let practice = load_exercises(&root, "practice")?;

    match command {
        CommandLine::Start => start(&root, &practice, false)?,
        CommandLine::Concept => {
            let concept = load_exercises(&root, "concept")?;
            start(&root, &concept, false)?;
        }
        CommandLine::Watch => start(&root, &practice, true)?,
        CommandLine::List => list(&practice),
        CommandLine::Show => {
            let current = find_current(&root, &practice)?;
            show_exercise(current)?;
        }
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

fn start(
    root: &Path,
    exercises: &[ExerciseInfo],
    watch: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_index = 0;
    while current_index < exercises.len() {
        let current = &exercises[current_index];
        print_progress(exercises, current_index);
        show_exercise(current)?;

        let passed = run_test(root, current)?;
        if passed {
            println!("✅ {} 已通过！", current.exercise.name);
            current_index += 1;
            if current_index < exercises.len() {
                println!("➡️ 下一题：{}", exercises[current_index].exercise.name);
            }
            continue;
        }

        if watch {
            watch_exercise(root, current)?;
            continue;
        }

        println!("当前题目尚未通过，修改后再次运行 `cargo start`。");
        break;
    }

    if current_index == exercises.len() {
        println!("🎉 全部 {} 道 practice 练习已通过！", exercises.len());
    }
    Ok(())
}

fn find_current<'exercise>(
    root: &Path,
    exercises: &'exercise [ExerciseInfo],
) -> Result<&'exercise ExerciseInfo, Box<dyn std::error::Error>> {
    let mut current_index = 0;
    while current_index < exercises.len() {
        if !run_test(root, &exercises[current_index])? {
            return Ok(&exercises[current_index]);
        }
        current_index += 1;
    }
    exercises.last().ok_or_else(|| "没有找到 practice 练习".into())
}

fn run_test(
    root: &Path,
    exercise: &ExerciseInfo,
) -> Result<bool, Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .current_dir(root)
        .args(["test", "-p", &exercise.package_name])
        .status()?;
    Ok(status.success())
}

fn watch_exercise(
    root: &Path,
    exercise: &ExerciseInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("👀 正在监听 {}，保存文件后自动测试。", exercise.exercise.name);
    let (sender, receiver) = channel::<notify::Result<Event>>();
    let mut watcher = RecommendedWatcher::new(
        move |event| {
            let _ = sender.send(event);
        },
        Config::default(),
    )?;
    watcher.watch(&exercise.directory, RecursiveMode::Recursive)?;

    loop {
        match receiver.recv_timeout(Duration::from_millis(500)) {
            Ok(Ok(event)) if is_source_change(&event) => {
                io::stdout().flush()?;
                println!("\n🔄 检测到文件变化，重新测试...");
                if run_test(root, exercise)? {
                    println!("✅ {} 已通过！", exercise.exercise.name);
                    return Ok(());
                }
            }
            Ok(Err(error)) => println!("监听失败：{error}"),
            Ok(Ok(_)) | Err(_) => {}
        }
    }
}

fn is_source_change(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
    )
}

fn print_progress(exercises: &[ExerciseInfo], current_index: usize) {
    let completed = current_index;
    let total = exercises.len();
    println!("\n进度：{completed}/{total}（{:.0}%）", completed as f64 / total as f64 * 100.0);
    println!("当前：{}", exercises[current_index].exercise.name);
}

fn show_exercise(exercise: &ExerciseInfo) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n========== {} ==========", exercise.exercise.name);
    for file_name in [
        ".docs/instructions.md",
        ".docs/introduction.md",
        ".docs/hints.md",
        ".docs/instructions.append.md",
    ] {
        let path = exercise.directory.join(file_name);
        if path.exists() {
            println!("\n----- {} -----\n", path.display());
            println!("{}", fs::read_to_string(path)?);
        }
    }
    Ok(())
}

fn list(exercises: &[ExerciseInfo]) {
    for (index, exercise) in exercises.iter().enumerate() {
        let status = exercise.exercise.status.as_deref().unwrap_or("active");
        println!(
            "{:>3}. 难度 {:>2} [{}] {}",
            index + 1,
            exercise.exercise.difficulty,
            status,
            exercise.exercise.name
        );
    }
}

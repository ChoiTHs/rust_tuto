use clap::Parser;
use colored::*;
use walkdir::WalkDir;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Instant;

/// 명령행 인자 구조체
#[derive(Parser)]
#[command(name = "my_cli_tool", about = "파일 및 폴더 검색 CLI 도구", version = "1.0")]
struct Args {
    /// 검색할 문자열
    query: String,

    /// 검색할 디렉토리 (기본값: 현재 디렉토리)
    #[arg(default_value = "D:\\")]
    directory: String,
}

/// 검색된 항목이 속한 상위 디렉토리를 포함한 트리 출력
fn search_files(directory: &str, query: &str) {
    let path = Path::new(directory);
    let start_time = Instant::now(); // ⏳ 작업 시작 시간 측정

    if !path.exists() || !path.is_dir() {
        println!("{}", format!("❌ 오류: '{}' 는 존재하지 않거나 디렉토리가 아닙니다.", directory).red());
        return;
    }

    println!("{}", format!("🔍 '{}' 디렉토리에서 '{}' 검색 중...\n", directory, query).cyan());

    let mut tree_structure: BTreeMap<PathBuf, Vec<(PathBuf, bool)>> = BTreeMap::new();
    let mut seen_dirs: HashSet<PathBuf> = HashSet::new();

    // 모든 파일과 폴더를 검색하여 저장
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_path = entry.path().to_path_buf();
        let parent_path = file_path.parent().unwrap_or(Path::new(directory)).to_path_buf();
        let is_dir = file_path.is_dir();

        // 검색된 파일/폴더가 `query`를 포함하면 추가
        if file_name.contains(query) {
            tree_structure.entry(parent_path.clone()).or_insert(vec![]).push((file_path.clone(), is_dir));

            // 상위 폴더도 포함하여 출력되도록 설정
            let mut current = parent_path.clone();
            while current != Path::new(directory) && seen_dirs.insert(current.clone()) {
                let grandparent = current.parent().unwrap_or(Path::new(directory)).to_path_buf();
                tree_structure.entry(grandparent.clone()).or_insert(vec![]).push((current.clone(), true));
                current = grandparent;
            }
        }
    }

    if !tree_structure.is_empty() {
        print_tree(&tree_structure, Path::new(directory), 0, true);
    } else {
        println!("{}", format!("❌ '{}' 검색 결과 없음.", query).red());
    }

    let duration = start_time.elapsed(); // ⏳ 작업 종료 시간 측정
    println!("\n✅ 검색 완료! 총 소요 시간: {:.2?} 초", duration.as_secs_f64());
}

/// 트리 구조 출력 함수 (루트 경로 출력 개선 + 중복 제거)
fn print_tree(
    tree: &BTreeMap<PathBuf, Vec<(PathBuf, bool)>>,
    root: &Path,
    depth: usize,
    is_root: bool,
) {
    if let Some(entries) = tree.get(root) {
        let mut entries = entries.clone();
        entries.sort_by(|a, b| a.0.cmp(&b.0)); // 이름 기준 정렬

        if is_root {
            println!("{}", root.display().to_string().bold());
        }

        for (index, (entry, is_dir)) in entries.iter().enumerate() {
            let is_last = index == entries.len() - 1;
            let prefix = format!(
                "{}{}── ",
                "│   ".repeat(depth),
                if is_last { "└" } else { "├" }
            );

            if *is_dir {
                println!("{}📁 {}", prefix.blue().bold(), entry.file_name().unwrap().to_string_lossy().blue().bold());
                print_tree(tree, entry, depth + 1, false);
            } else {
                println!("{}📄 {}", prefix, entry.file_name().unwrap().to_string_lossy());
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    search_files(&args.directory, &args.query);
}

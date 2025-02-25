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

    // 입력된 디렉토리가 존재하지 않거나 유효하지 않은 경우 오류 출력 후 종료
    if !path.exists() || !path.is_dir() {
        println!("{}", format!("❌ 오류: '{}' 는 존재하지 않거나 디렉토리가 아닙니다.", directory).red());
        return;
    }

    println!("{}", format!("🔍 '{}' 디렉토리에서 '{}' 검색 중...\n", directory, query).cyan());

    // 트리 구조를 저장할 BTreeMap (정렬된 맵)
    let mut tree_structure: BTreeMap<PathBuf, Vec<(PathBuf, bool)>> = BTreeMap::new();
    let mut seen_dirs: HashSet<PathBuf> = HashSet::new(); // 이미 추가된 디렉토리 중복 방지

    // WalkDir을 사용해 지정된 디렉토리 내 모든 파일 및 폴더를 순회하며 검색
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_string_lossy().to_string(); // 파일 이름을 문자열로 변환
        let file_path = entry.path().to_path_buf(); // 파일의 전체 경로 저장
        let parent_path = file_path.parent().unwrap_or(Path::new(directory)).to_path_buf(); // 상위 디렉토리 경로
        let is_dir = file_path.is_dir(); // 현재 항목이 디렉토리인지 확인

        // 파일명 또는 폴더명이 검색어(query)를 포함하는 경우
        if file_name.contains(query) {
            // 검색 결과를 트리 구조에 추가
            tree_structure.entry(parent_path.clone()).or_insert(vec![]).push((file_path.clone(), is_dir));

            // 상위 폴더들도 출력될 수 있도록 추가
            let mut current = parent_path.clone();
            while current != Path::new(directory) && seen_dirs.insert(current.clone()) {
                let grandparent = current.parent().unwrap_or(Path::new(directory)).to_path_buf();
                tree_structure.entry(grandparent.clone()).or_insert(vec![]).push((current.clone(), true));
                current = grandparent;
            }
        }
    }

    // 검색 결과가 있는 경우 트리 형태로 출력
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

        // 루트 디렉토리인 경우 경로를 볼드 처리하여 출력
        if is_root {
            println!("{}", root.display().to_string().bold());
        }

        for (index, (entry, is_dir)) in entries.iter().enumerate() {
            let is_last = index == entries.len() - 1; // 마지막 항목 여부 확인
            let prefix = format!(
                "{}{}── ",
                "│   ".repeat(depth),
                if is_last { "└" } else { "├" }
            );

            // 폴더와 파일을 구분하여 출력
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
    let args = Args::parse(); // 명령행 인자 파싱
    search_files(&args.directory, &args.query); // 검색 실행
}

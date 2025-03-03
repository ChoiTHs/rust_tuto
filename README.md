# 🦀 Rust Projects Repository

이 저장소는 **Rust 언어로 개발한 다양한 프로젝트들을 모아둔 공간**입니다.  
Rust를 학습하고 실습하며 만든 여러 프로젝트들을 정리하고 관리하는 목적을 가지고 있습니다.

---

## 📌 이 저장소의 목적
- Rust 프로그래밍 언어를 학습하고 실습한 내용을 기록합니다.
- Rust로 만든 다양한 CLI, GUI, 웹 애플리케이션, 게임 등을 저장합니다.
- 새로운 Rust 프로젝트를 추가하며, 학습한 내용을 꾸준히 공유합니다.

---

## 📂 현재 포함된 프로젝트

| 프로젝트명 | 설명 | 실행 방법 |
|------------|---------------------------|--------------------------------|
| [my_cli_tool](./my_cli_tool)| CLI에서 특정 단어가 포함된 파일/폴더 검색 | `cargo run -- "검색어"` |
| [Todo_List_json](./Todo_List_json)| Todo List를 json형식으로 저장하는 방식 | `cargo run -- add "Todo List 만들기"` |
| [Todo_List_SQLite](./todo_list_SQLite/)| Todo List를 SQLite를 사용해 저장하는 방식 | `cargo run -- add "Todo List 써보기"` |
| (추가 예정) | Rust로 개발한 추가 프로젝트 | - |

---

## 🚀 Rust 프로젝트 실행 방법

모든 Rust 프로젝트는 `cargo`를 사용하여 실행할 수 있습니다.  
다음 명령어를 실행하면 Rust 프로젝트를 빌드하고 실행할 수 있습니다.

```bash
cargo build --release
cargo run --help
```

---

## 🖥️ 이 저장소를 사용하는 방법
### ✅ 1. 저장소 클론
GitHub에서 이 저장소를 클론하여 로컬 환경에서 실행할 수 있습니다.
```bash
git clone https://github.com/your-username/rust-projects.git
cd rust-projects
```

### ✅ 2. Rust 및 Cargo 설치
Rust가 설치되어 있지 않다면, 공식 사이트에서 설치하세요.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ✅ 3. 프로젝트 실행
저장소 안의 개별 프로젝트 디렉토리로 이동하여 실행하세요.
```bash
cd my_cli_tool
cargo run -- "검색어"
```

---

## 📝 Rust 학습 진행 상황
- ✅ Rust 기본 문법 익히기 (let, mut, struct, enum 등)
- ✅ Cargo 패키지 관리 (Cargo.toml, dependencies)
- ✅ CLI 애플리케이션 개발 (clap, colored, walkdir 활용)
- ⏳ GUI 프로젝트 추가 예정
- ⏳ 웹 애플리케이션 프로젝트 추가 예정
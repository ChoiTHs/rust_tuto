# CLI 파일 및 폴더 검색 도구

Rust로 만든 간단한 CLI 도구로, 특정 단어가 포함된 파일 및 폴더를 검색하고 트리 형식으로 출력합니다.

---

## 📌 프로젝트 개요
이 프로젝트는 Rust로 개발된 CLI 도구로, 사용자가 입력한 **검색어(Query)**를 포함하는 파일과 폴더를 **Windows의 특정 디렉토리(기본: `D:\` 드라이브)**에서 찾아 트리 구조로 출력합니다.

**주요 기능:**
- 특정 단어가 포함된 파일 및 폴더 검색
- 검색된 파일과 폴더를 트리 형식으로 출력
- 검색에 걸린 시간을 표시
- Windows 환경에서 동작하도록 최적화됨

---

## 🛠️ 사용한 기술
- **Rust**: CLI 애플리케이션 개발
- **Cargo**: Rust 패키지 관리 및 빌드
- **Crate (라이브러리) 사용**
  - [`clap`](https://docs.rs/clap/latest/clap/) → CLI 인자 파싱
  - [`colored`](https://docs.rs/colored/latest/colored/) → 터미널 색상 추가
  - [`walkdir`](https://docs.rs/walkdir/latest/walkdir/) → 디렉토리 재귀 탐색
  - `std::time::Instant` → 실행 시간 측정

---

## 🎯 배운 점
이 프로젝트를 통해 다음과 같은 개념을 학습하고 경험했습니다:
- **Rust의 기본 문법 및 프로젝트 구조** (`main.rs`, `Cargo.toml`)
- **CLI 프로그램에서 사용자 입력 처리 (`clap` 사용)**
- **파일 시스템 탐색 (`std::fs` 및 `walkdir`)**
- **Rust에서 해시맵(`HashMap`, `BTreeMap`)을 사용하여 폴더 구조 정리**
- **Rust에서 컬러 출력 (`colored` 사용)**
- **Rust에서 실행 시간 측정 (`Instant::now()`)**

---

## 🚀 설치 및 실행 방법
### 1️⃣ **Rust 및 Cargo 설치**
먼저, Rust와 Cargo가 설치되어 있는지 확인하세요.
```bash
rustc --version
cargo --version
```
만약 설치되지 않았다면, 아래 명령어를 사용하여 설치합니다.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2️⃣ 프로젝트 클론 및 빌드
```bash
git clone https://github.com/your-repository/my_cli_tool.git
cd my_cli_tool
cargo build --release
```
빌드가 완료되면 실행 파일이 target/release/my_cli_tool.exe에 생성됩니다.

###
기본 실행 (D:\ 드라이브에서 "rust" 검색)
```bash
cargo run -- "rust"
```

특정 폴더 내에서 검색 (D:\dev 폴더에서 "rust" 검색)
```bash
cargo run -- "rust" "D:\dev"
```

cargo install을 사용하여 어디서든 실행 가능하게 설정
```bash
cargo install --path .
```

이제 my_cli_tool을 명령어로 실행할 수 있습니다.
```bash
my_cli_tool "rust" "D:\"
```

---

## 🖥️ Windows에서 실행하는 방법
### ✅ 1. 환경 변수 설정
Rust 프로젝트를 Windows에서 편하게 실행하려면, CARGO_HOME 및 PATH 환경 변수를 설정해야 합니다.

1. PowerShell에서 실행
    ```powershell
    $env:PATH += ";C:\Users\USERNAME\.cargo\bin"
    ```
2. Windows 환경 변수에 추가
    - 시스템 속성 > 고급 시스템 설정 > 환경 변수
    - PATH에 C:\Users\USERNAME\.cargo\bin 추가

### ✅ 2. 빌드 및 실행
Windows에서는 .exe 파일을 직접 실행할 수도 있습니다.
```powershell
target\release\my_cli_tool.exe "rust" "D:\dev"
```

## 📢 개발자 노트
이 프로젝트는 Rust CLI 애플리케이션을 처음 만들어보며 학습한 내용들을 바탕으로 개발되었습니다.
추후 기능 개선이 필요하면 업데이트할 예정입니다!
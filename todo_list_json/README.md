# 📌 Rust CLI Todo List

## 📖 소개
Rust로 제작한 간단한 CLI 기반 Todo List 프로그램입니다.
할 일을 추가, 조회, 완료 처리, 삭제할 수 있으며, 데이터를 JSON 파일에 저장하여 유지합니다.

---

## 🚀 설치 및 실행 방법

### 1️⃣ 프로젝트 클론 및 의존성 설치
```sh
# Git 저장소 클론
git clone https://github.com/your-repo/todo-rust.git
cd todo-rust

# Rust 패키지 설치 및 빌드
cargo build
```

### 2️⃣ 실행 방법
```sh
cargo run -- [명령어] [옵션]
```

---

## 📌 사용 방법

### 1️⃣ 할 일 추가
새로운 할 일을 목록에 추가합니다.
```sh
cargo run -- add "Rust 배우기"
```
✅ 예시 출력:
```
Added: Rust 배우기
```

### 2️⃣ 할 일 목록 조회
현재 저장된 모든 할 일을 출력합니다.
```sh
cargo run -- list
```
✅ 예시 출력:
```
1. [ ] - Rust 배우기
2. [✔] - 프로젝트 완성하기
```

### 3️⃣ 할 일 완료 처리
특정 번호의 할 일을 완료 상태로 변경합니다.
```sh
cargo run -- done 1
```
✅ 예시 출력:
```
Marked as done: Rust 배우기
```

### 4️⃣ 할 일 삭제
특정 번호의 할 일을 삭제합니다.
```sh
cargo run -- delete 1
```
✅ 예시 출력:
```
Deleted todo 1
```

---

## 📝 데이터 저장 방식
- 모든 할 일 목록은 `todo.json` 파일에 저장됩니다.
- 프로그램이 실행될 때 기존 데이터를 불러오며, 종료 후에도 데이터가 유지됩니다.

예제 `todo.json` 파일:
```json
[
    { "id": 1, "title": "Rust 배우기", "completed": false },
    { "id": 2, "title": "프로젝트 완성하기", "completed": true }
]
```

---

## 🛠️ 추가 개선 사항 (To-Do)
- ✅ SQLite를 사용하여 데이터 저장
- ✅ 할 일 검색 기능 추가
- ✅ GUI 지원 (Tauri or egui)
- ✅ 웹 API 서버로 확장 (Axum or Rocket 활용)

---

## 📌 라이선스
이 프로젝트는 MIT 라이선스를 따릅니다.


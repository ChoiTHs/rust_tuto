# 📝 Rust Todo List (SQLite 버전)

SQLite를 이용한 Rust CLI Todo List 프로그램입니다.
할 일 목록을 데이터베이스(SQLite)에 저장하며, 프로그램을 종료해도 데이터가 유지됩니다.
---

## 🚀 1. 프로젝트 설정 및 실행 방법
### 1️⃣ 프로젝트 클론 및 빌드
```bash
git clone https://github.com/your-repo/todo_list_db.git
cd todo_list_db
cargo build
```

### 2️⃣ 데이터베이스 자동 생성
- SQLite 데이터베이스는 프로그램 실행 시 자동 생성됩니다.
- 수동으로 초기화하려면:
  ```sh
  sqlite3 todo.db < db/init.sql
  ```

### 3️⃣ 실행 방법
```sh
cargo run -- [명령어] [옵션]
```

---

## 📌 2. 사용 가능한 명령어

|명령어|설명|사용법|
|-------|-------|-------|
|add|새로운 할 일 추가|cargo run -- add "할 일 내용"|
|list|현재 저장된 모든 할 일 조회|cargo run -- list|
|done|특정 번호의 할 일을 완료 처리|cargo run -- done [할 일 ID]|
|delete|특정 번호의 할 일을 삭제|cargo run -- delete [할 일 ID]|

---

## 🛠 3. 사용 예시
### ✅ 1) 할 일 추가
```sh 
cargo run -- add "Rust 배우기"
```
출력 예시:
  ```makefile
  Added: Rust 배우기
  ```

### ✅ 2) 할 일 목록 조회
```sh
cargo run -- list
```
출력 예시:
  ```css
  1. [ ] - Rust 배우기
  ```

### ✅ 3) 할 일 완료 처리
```sh
cargo run -- done 1
```
출력 예시:
  ```bash
  Marked as done: Rust 배우기
  ```

### ✅ 4) 할 일 삭제
```sh
cargo run -- delete 1
```
출력 예시:
  ```nginx
  Deleted todo 1
  ```

---

## 📂 4. 데이터 저장 방식
모든 할 일 데이터는 todo.db (SQLite 파일) 안에 저장됩니다.

### ✅ 데이터베이스 직접 조회
SQLite CLI에서 데이터 확인:
  ```sh
  sqlite3 todo.db
  SELECT * FROM todos;
  ```


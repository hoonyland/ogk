# OGK - [open.go.kr <대한민국 정보공개플랫폼>](https://open.go.kr) Client
API Server 및 CLI 의 형태로 [정보공개플랫폼](https://open.go.kr) 계정이 있고 를랫폼을 자주 사용하는 사용자라면 누구나 편리하게 서비스를 이용할 수 있도록 돕기 위한 클라이언트입니다.

## 일반 사용자
### API
> TODO

### CLI(Command Line Interface)
```bash
ogk help
```

#### 0-1. 인증하기
```bash
# username, password 이 성공적으로 인증되면
# $HOME/.ogk/credentials 파일에 toml 포맷으로 저장됩니다.
# 이후 open.go.kr API 통신이 필요한 경우 자동으로 저장된 사용자 정보를 활용합니다.
ogk auth login --username <username> --password <password>
```

#### 0-2. 설정하기
```bash
# 정보공개청구 후 전달받은 파일을 관리할 Github Repository
ogk config repository --file <repository-url-for-files>

# 정보공개청구 내역을 저장하고 관리할 Database 정보
ogk config database --host <db-host> --username <db-username> --password <db-password>

# 설정 확인하기
ogk config list
```

#### 1. 조회하기

- 단건 조회하기
```bash
ogk fetch --bill-id <bill-id>

# 청구파일 저장 옵션
ogk fetch --bill-id <bill-id> --download

# 청구 결과 예쁘게 보기 
ogk fetch --bill-id <bill-id> | jq .
ogk fetch --bill-id <bill-id> | jq .dtlVo
ogk fetch --bill-id <bill-id> | jq .dtlVo.rceptDt
```

- 복수건 조회하기(날짜 조회)
```bash
ogk fetch --page 1 --from-date 2021-01-01 --to-date 2020-12-31
# --page, -p (Optional) 페이지
# --from-date, -f (Optional) 시작일
# --to-date, -t (Optional) 마지막일
```

#### 2. 파일 관리하기
> TODO

#### 3. 데이터 관리하기
> TODO


## 개발자 & 기여자

### 0. 사용하고 있는 언어 및 도구
- [Rust](https://www.rust-lang.org/)
- [reqwest](https://docs.rs/reqwest/0.10.10/reqwest): HTTP Client
- [rocket](https://rocket.rs/): Web Framework
- [diesel](https://diesel.rs/): Database ORM
- [clap](https://docs.rs/clap/3.0.0-beta.2/clap): CLI 빌더

### 1. 시작하기
#### 크롤러 개발하기
> TODO

#### API 개발하기
> TODO

#### CLI 개발하기
> TODO
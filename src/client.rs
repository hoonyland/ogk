#![allow(non_snake_case)]

use crate::dm::models::Bill;
use crate::utils::auth::AuthConfig;
use bytes::Bytes;
use chrono::prelude::*;
use futures::Future;
use reqwest::{header, Error, Response};
use std::pin::Pin;
use std::str;

const LIST_HOST: &str = "https://www.open.go.kr/rqestMlrd/rqestDtls/reqstDocSrchList.ajax";
const LOGIN_HOST: &str = "https://www.open.go.kr/com/login/memberLogin.ajax";
const DETAIL_HOST: &str = "https://www.open.go.kr/rqestMlrd/rqestDtls/reqstDocDecsnNotie.do";
const DOWNLOAD_HOST: &str = "https://www.open.go.kr/util/FileDownload.do";

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponseModelAndViewModelResultRtnV0 {
  pub accesType: String,
  pub addr1: String,
  pub addr2: String,
  pub age: i32,
  pub agent: String,
  pub agentInfo: String,
  pub apoloId: String,
  pub birth: String,
  pub birthDe: String,
  pub bizrNo: String,
  pub bizrNo1: String,
  pub bizrNo2: String,
  pub bizrNo3: String,
  pub changePwdYn: String,
  pub crt: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponseModelAndViewModelResult {
  pub error_code: String,
  pub error_msg: String,
  pub mberSeCd: String,
  pub sysdate: String,
  pub today: String,
  // pub rtnV0: AuthResponseModelAndViewModelResultRtnV0,
}

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponseModelAndViewModel {
  pub result: AuthResponseModelAndViewModelResult,
}

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponseModelAndView {
  pub empty: bool,
  pub model: AuthResponseModelAndViewModel,
}

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponse {
  pub modelAndView: AuthResponseModelAndView,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ListVo {
  pub totalPage: i32, // 아이템 전체 개수
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DtlVo {
  pub clsdrResnCn: String,        // 비공개내용
  pub decsnCn: String,            // 공개내용 ex)
  pub insttRqestProcStCd: String, // 처리상태 코드 ex) 143
  pub insttRqestProcStNm: String, // 처리상태명 ex) 공개완료
  pub mberId: String,             // 사용자이름 // ex) opengirok
  pub nticeDt: String,            // 처리일자 ex) 2020.07.27
  pub othbcDtApnResnNm: String,   // 공개일시 지정 사유 ex) 수수료납부 완료후 바로 공개
  pub othbcSeNm: String,          // 공개여부 ex) 공개
  pub prcsFullInsttNm: String,    // 처리기관 이름 full ver ex) 고용노동부 최저임금위원회
  pub prcsInsttCd: String,        // 처리기관 코드 ex) 1492865
  pub prcsInsttNm: String,        // ex) 처리기관 이름 short ver. - 최저임금위원회
  pub procRegstrNo: String,       // * 세부 페이지 요청에 필요한 번호
  pub rceptDt: String,            // 접수일자 ex)  2020.09.12
  pub rqestCn: String,            // 청구내용 ex)
  pub rqestFullInsttNm: String,   // ex) 요청기관 이름 full ver. - 고용노동부 최저임금위원회
  pub rqestInsttCd: String,       // 요청기관 코드 ex) 1492865
  pub rqestInsttNm: String,       // ex) 요청기관 이름 short ver. - 최저임금위원회
  pub rqestProcRegstrNo: String,  // * 세부 페이지 요청에 필요한 번호
  pub rqestRceptNo: String,       // 접수번호 * 세부 페이지 요청에 필요한 번호
  pub rqestSj: String,            // 요청 제목 ex) 최저임금 위원회 회의록 및 속기록 (JE)
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DntcFile {
  pub atchmnflByteCo: String,     // '100081',
  pub atchmnflPrsrvNm: String,    // '202007171546284220000.zip',
  pub csdCnvrStCd: String,        // '020',
  pub fileAbsltCoursNm: String,   // '/pidfiles/uploads/pb/dlsrinfo/',
  pub fileSn: String,             // '1',
  pub fileUploadNo: String,       // 'VVdXZnJWYWI5Mm5GTzlsN1dWdno0QT09',
  pub frstRegisterId: String,     // 'MIG',
  pub uploadFileOrginlNm: String, // ex) '서범수 의원 요구자료 일체.zip',
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct BillWithFiles {
  pub atchFileList: Option<Vec<DntcFile>>,
  pub dntcFileList: Option<Vec<DntcFile>>,
  pub dtlVo: DtlVo,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Bills {
  pub list: Vec<DtlVo>,
  pub vo: ListVo,
}

#[derive(Debug)]
pub struct Client {
  client: reqwest::Client,
}

impl Client {
  pub fn new() -> Pin<Box<Self>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
      "Accept",
      "application/json, text/javascript, */*; q=0.01"
        .parse()
        .unwrap(),
    );
    headers.insert(
      "Content-Type",
      "application/x-www-form-urlencoded; charset=UTF-8"
        .parse()
        .unwrap(),
    );
    headers.insert("Host", "www.open.go.kr".parse().unwrap());
    headers.insert("Origin", "https://www.open.go.kr".parse().unwrap());
    headers.insert(
      "User-Agent",
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:80.0) Gecko/20100101 Firefox/80.0"
        .parse()
        .unwrap(),
    );

    let client = reqwest::ClientBuilder::new()
      .default_headers(headers)
      .cookie_store(true)
      .build()
      .unwrap();

    Box::pin(Client { client })
  }

  pub async fn auth(
    &self,
    username: &str,
    password: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let auth: [(&str, &str); 3] = [("mberId", username), ("pwd", password), ("agent", "PC")];
    let response = self.client.post(LOGIN_HOST).form(&auth).send().await?;
    match response.json::<AuthResponse>().await {
      Ok(response_json) => {
        if response_json.modelAndView.model.result.error_msg == "로그인 완료" {
          return Ok(());
        }

        panic!("사용자이름과 비밀번호를 확인해주세요.");
      }
      Err(_) => {
        panic!("사용자이름과 비밀번호를 확인해주세요.");
      }
    }
  }

  pub async fn auth_from_storage(&self) -> Result<(), Box<dyn std::error::Error>> {
    let config = AuthConfig::load()?;
    let decoded_password = base64::decode(&config.default.password.as_bytes())?;
    let password = str::from_utf8(&decoded_password)?;
    self.auth(&config.default.username, password).await?;
    Ok(())
  }

  // pub async fn download_file(
  //   &self,
  //   file: &DntcFile,
  // ) -> impl futures::Future<Output = Bytes, Error> {
  //   let params = &[("fileUploadNo", &file.fileUploadNo)];
  //   self
  //     .client
  //     .post(DOWNLOAD_HOST)
  //     .form(params)
  //     .send()
  //     .await?
  //     .bytes()
  // }

  pub async fn post(&self, url: &str, form: &[(&str, &str)]) -> Result<Response, Error> {
    self.client.post(url).form(form).send().await
  }

  pub async fn fetch_a_bill(
    &self,
    rqest_rcept_no: &str,
    rqest_proc_regstr_no: &str,
    proc_regstr_no: &str,
  ) -> Result<Option<BillWithFiles>, Error> {
    let params: [(&str, &str); 6] = [
      ("rqestRceptNo", rqest_rcept_no),
      ("rqestProcRegstrNo", rqest_proc_regstr_no),
      ("procRegstrNo", proc_regstr_no),
      ("deptSn", "1"), // 첨부파일 포함 옵션
      ("hash", "true"),
      ("multiDeptProcYn", "Y"),
    ];

    let response = self.post(DETAIL_HOST, &params).await?;
    let text_response = response.text().await?;
    // TODO: start_index & end_index 로 파일 여부에 따른 핸들링 리팩토링 필요
    let mut start_index = 0;
    let mut end_index = 0;
    let start_index_result = text_response.find("var result     	= {");
    match start_index_result {
      None => {}
      Some(index) => start_index = index,
    }

    let end_index_result = text_response.find("}};");
    match end_index_result {
      None => {}
      Some(index) => end_index = index,
    }

    if start_index > 0 && end_index > 0 {
      let stringified_json_result = &text_response[(start_index + 18)..(end_index + 2)];
      Ok(Some(serde_json::from_str(stringified_json_result).unwrap()))
    } else {
      Ok(None)
    }
  }

  pub async fn fetch_bills(
    &self,
    page: &str,
    from_date: &str,
    to_date: &str,
  ) -> Result<Bills, Error> {
    let params: [(&str, &str); 8] = [
      ("stRceptDt", from_date),
      ("edRceptDt", to_date),
      ("searchYn", "Y"),
      ("selRowPage", "10"),
      ("moveStatus", "L"),
      ("viewPage", page),
      ("rowPage", "10"),
      ("chkDate", "nonClass"),
    ];

    let response = self.client.post(LIST_HOST).form(&params).send().await?;
    response.json::<Bills>().await
  }

  pub fn fetch_bills_all(&self, page: &str) -> impl Future<Output = Result<Response, Error>> {
    let dt: DateTime<Utc> = Utc::now();
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9 * 3600)); // KST +09:00
    let today = fixed_dt.format("%Y-%m-%d");

    let params: [(&str, &str); 8] = [
      ("stRceptDt", "2009-01-01"),
      ("edRceptDt", &today.to_string()),
      ("searchYn", "Y"),
      ("selRowPage", "10"),
      ("moveStatus", "L"),
      ("viewPage", page),
      ("rowPage", "10"),
      ("chkDate", "nonClass"),
    ];

    self.client.post(LIST_HOST).form(&params).send()
  }
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[tokio::test]
  #[ignore]
  async fn test_auth_password() {
    let client = Client::new();
    todo!();
    // FIXME: Please insert a corrent user's information to authenticate
    let username = "";
    let password = "";
    let result = client.as_ref().auth(username, password).await.unwrap();
    assert_eq!(true, false)
  }

  #[tokio::test]
  #[ignore]
  async fn test_auth_wrong_password() {
    let client = Client::new();
    let username = "test";
    let password = "passw0rd";
    let result = client.as_ref().auth(username, password).await.unwrap();
    assert_eq!(false, false)
  }

  #[tokio::test]
  #[ignore]
  async fn test_fetch_a_bill() {
    let client = generate_dummy_client().await;
    let response = client
      .as_ref()
      .fetch_a_bill("1436948", "1436948", "1272006")
      .await
      .unwrap();

    if let Some(result) = response {
      assert_eq!("1436948", result.dtlVo.rqestProcRegstrNo)
    }
  }

  #[ignore]
  async fn generate_dummy_client() -> Pin<Box<Client>> {
    let client = Client::new();
    let username = "opengirok";
    let password = "";
    let _ = client.as_ref().auth(username, password).await.unwrap();
    client
  }
}

use crate::crawler::{DntcFile, DtlVo};
use crate::dm::models;
use bytes::Bytes;
use git2::{Commit, Cred, IndexAddOption, ObjectType, Oid, RemoteCallbacks, Repository, Signature};
use regex::Regex;
use std::env;
use std::fs::{create_dir, remove_dir_all, File};
use std::io;
use std::path::Path;

pub struct FileManager<'a> {
  _remote_url: String,
  _remote_url_public: String,
  _local_path: String,
  _local_path_public: String,
  _local_repo: Option<Repository>,
  _local_repo_public: Option<Repository>,
  _git_signature: Signature<'a>,
}

impl<'a> FileManager<'a> {
  pub fn new(user: &models::User) -> Self {
    println!("{:?}", &user);
    // TODO: https 방식도 고려
    let _remote_url = format!("git@github.com:realopen/data-{}", &user.username);
    let _remote_url_public = format!("git@github.com:realopen/data-{}-public", &user.username);
    let _local_path = format!(
      "{}/Projects/personal/realopen-data/{}",
      env::var("HOME").unwrap(),
      &user.username
    );
    let _local_path_public = format!(
      "{}/Projects/personal/realopen-data/{}-public",
      env::var("HOME").unwrap(),
      &user.username
    );

    Self {
      _local_path: _local_path,
      _local_path_public: _local_path_public,
      _remote_url: _remote_url,
      _remote_url_public: _remote_url_public,
      _local_repo: None,
      _local_repo_public: None,
      _git_signature: Signature::now("bot-realopen", "the6thm0nth@outlook.com").unwrap(),
    }
  }

  pub fn clone_remote_repo(&mut self) -> (&Option<Repository>, &Option<Repository>) {
    let _ = remove_dir_all(&self._local_path);
    let _ = remove_dir_all(&self._local_path_public);

    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
      Cred::ssh_key(
        username_from_url.unwrap(),
        None,
        // TODO: ssh key 관리 고려
        std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
        None,
      )
    });

    // callbacks.sideband_progress(|progress| {
    //   println!("Cloning - {:?}%", progress);
    //   true
    // });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    match builder.clone(&self._remote_url, Path::new(&self._local_path)) {
      Ok(repo) => {
        self._local_repo = Some(repo);
      }
      Err(error) => {
        println!("{}", &self._remote_url);
        println!("{}", &self._local_path);
        panic!("{}", error);
      }
    }

    match builder.clone(
      &self._remote_url_public,
      Path::new(&self._local_path_public),
    ) {
      Ok(repo) => {
        self._local_repo_public = Some(repo);
      }
      Err(error) => {
        println!("{}", &self._remote_url_public);
        println!("{}", &self._local_path_public);
        panic!("{}", error);
      }
    }

    (&self._local_repo, &self._local_repo_public)
  }

  // {접수일자}_{청구_제묵}
  fn make_dirname(&self, bill: &models::Bill) -> String {
    let re_illegal_symbols = Regex::new("[.\"\n \t()\'~]").unwrap();
    let re_retouch = Regex::new("_+").unwrap();
    format!(
      "{}_{}",
      bill.request_date.replace(".", "-"),
      re_retouch
        .replace_all(
          &re_illegal_symbols.replace_all(&bill.request_subject.trim(), "_"),
          "_",
        )
        .to_string()
    )
  }

  // {접수일자}_{청구_제묵}
  fn make_dirname_by_row(&self, bill: &models::Bill) -> String {
    let re_illegal_symbols = Regex::new("[.\"\n \t()\'~]").unwrap();
    let re_retouch = Regex::new("_+").unwrap();
    format!(
      "{}_{}",
      bill.request_date.replace(".", "-"),
      re_retouch
        .replace_all(
          &re_illegal_symbols.replace_all(&bill.request_subject.trim(), "_"),
          "_",
        )
        .to_string()
    )
  }

  // {접수번호}_{처리기관이름}_{업로드_파일명}
  pub fn make_filename(bill: &models::Bill, file: &DntcFile) -> String {
    let re_illegal_symbols = Regex::new("[\"\n \t()\'~]").unwrap();
    let re_retouch = Regex::new("_+").unwrap();

    format!(
      "{}_{}_{}",
      bill.registration_number,
      bill.proc_org_name.replace(" ", ""),
      re_retouch
        .replace_all(
          &re_illegal_symbols.replace_all(&file.uploadFileOrginlNm.trim(), "_"),
          "_",
        )
        .to_string()
    )
  }

  // {접수번호}_{처리기관이름}_{업로드_파일명}
  pub fn make_filename_by_row(bill: &models::Bill, file: &DntcFile) -> String {
    let re_illegal_symbols = Regex::new("[\"\n \t()\'~]").unwrap();
    let re_retouch = Regex::new("_+").unwrap();

    format!(
      "{}_{}_{}",
      bill.registration_number,
      bill.proc_org_name.replace(" ", ""),
      re_retouch
        .replace_all(
          &re_illegal_symbols.replace_all(&file.uploadFileOrginlNm.trim(), "_"),
          "_",
        )
        .to_string()
    )
  }

  pub fn mkdir_by_bill(&self, bill: &models::Bill) -> std::io::Result<()> {
    let dir_path = format!("{}/{}", &self._local_path, &self.make_dirname(bill));
    create_dir(Path::new(&dir_path))?;
    let dir_path_public = format!("{}/{}", &self._local_path_public, &self.make_dirname(bill));
    create_dir(Path::new(&dir_path_public))?;
    Ok(())
  }

  pub fn mkdir_by_bill_row(&self, bill: &models::Bill) -> std::io::Result<()> {
    create_dir(Path::new(&self.make_dirname_by_row(bill)))?;
    Ok(())
  }

  pub fn save(
    &self,
    bill: &models::Bill,
    file: &DntcFile,
    downloaded_file: &Bytes,
    public: bool,
  ) -> File {
    let file_name = FileManager::make_filename(bill, file);
    let file_path = format!("{}/{}", self.make_dirname(bill), file_name);

    if public {
      let mut local_file =
        File::create(format!("{}/{}", &self._local_path_public, file_path)).unwrap();
      io::copy(&mut downloaded_file.as_ref(), &mut local_file).unwrap();
      local_file
    } else {
      println!("{}, {}", &self._local_path, &file_path);
      let mut local_file = File::create(format!("{}/{}", &self._local_path, file_path)).unwrap();
      io::copy(&mut downloaded_file.as_ref(), &mut local_file).unwrap();
      local_file
    }
  }

  pub fn save_by_row(&self, bill: &models::Bill, file: &DntcFile, downloaded_file: Bytes) -> File {
    let file_name = FileManager::make_filename_by_row(bill, file);
    let file_path = format!("{}/{}", self.make_dirname_by_row(bill), file_name);
    let mut local_file = File::create(file_path).unwrap();
    io::copy(&mut downloaded_file.as_ref(), &mut local_file).unwrap();
    local_file
  }

  pub fn upload(&self) -> Result<Oid, git2::Error> {
    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
      Cred::ssh_key(
        username_from_url.unwrap(),
        None,
        // TODO: ssh key 관리 고려
        std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
        None,
      )
    });

    fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
      let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
      obj
        .into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
    }

    let repo: &Repository;
    match &self._local_repo {
      Some(repository) => {
        repo = repository;
      }
      None => {
        panic!("No Local Repository");
      }
    }

    let mut index = repo.index().unwrap();
    index
      .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
      .unwrap();
    let oid = index.write_tree().unwrap();
    let parent_commit = find_last_commit(&repo).unwrap();
    let tree = repo.find_tree(oid).unwrap();

    repo
      .commit(
        Some("HEAD"),
        &self._git_signature,
        &self._git_signature,
        "hello world",
        &tree,
        &[&parent_commit],
      )
      .unwrap();

    let mut remote = match repo.find_remote("origin") {
      Ok(r) => r,
      Err(_) => repo.remote("origin", &self._remote_url)?,
    };
    // Prepare fetch options.
    let mut po = git2::PushOptions::new();
    let mut po = po.remote_callbacks(callbacks);
    remote.push(&["refs/heads/master"], Some(&mut po))?;

    Ok(oid)
  }
}

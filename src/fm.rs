// use crate::client::{DntcFile, DtlVo};
// use crate::dm::models;
use bytes::Bytes;
use dirs::home_dir;
use git2::{Commit, Cred, IndexAddOption, ObjectType, Oid, RemoteCallbacks, Repository, Signature};
use regex::Regex;
use std::env;
use std::fs::{create_dir, remove_dir_all, File};
use std::io;
use std::path::Path;

pub struct FileManager<'a> {
    _remote_url: String,
    _local_path: &'a str,
    _local_repo: Option<Repository>,
    _git_signature: Signature<'a>,
}

impl<'a> FileManager<'a> {
    pub fn new(local_repository: &'a str, remote_repository: &'a str) -> Self {
        // TODO: https 방식도 고려
        let _remote_url = format!("git@github.com:{}", remote_repository);

        let mut fm = FileManager {
            _local_path: local_repository,
            _remote_url: _remote_url,
            _local_repo: None,
            _git_signature: Signature::now("bot-realopen", "hoonyland.newsletter@gmail.com")
                .unwrap(),
        };

        if !Path::new(local_repository).exists() {
            fm.clone_remote_repo();
        }

        return fm;
    }

    pub fn clone_remote_repo(&mut self) -> &Option<Repository> {
        let _ = remove_dir_all(&self._local_path);

        // Prepare callbacks.
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                // TODO: ssh key 관리 고려
                std::path::Path::new(&format!(
                    "{}/.ssh/id_rsa",
                    home_dir().unwrap().to_str().unwrap()
                )),
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

        &self._local_repo
    }

    // {접수일자}_{청구_제묵}
    pub fn make_dirname(request_date: &str, request_subject: &str) -> String {
        let re_illegal_symbols = Regex::new("[.\"\n \t()\'~]").unwrap();
        let re_retouch = Regex::new("_+").unwrap();
        format!(
            "{}_{}",
            request_date.replace(".", "-"),
            re_retouch
                .replace_all(
                    &re_illegal_symbols.replace_all(request_subject.trim(), "_"),
                    "_",
                )
                .to_string()
        )
    }

    // // {접수일자}_{청구_제묵}
    // fn make_dirname_by_row(&self, bill: &models::Bill) -> String {
    //   let re_illegal_symbols = Regex::new("[.\"\n \t()\'~]").unwrap();
    //   let re_retouch = Regex::new("_+").unwrap();
    //   format!(
    //     "{}_{}",
    //     bill.request_date.replace(".", "-"),
    //     re_retouch
    //       .replace_all(
    //         &re_illegal_symbols.replace_all(&bill.request_subject.trim(), "_"),
    //         "_",
    //       )
    //       .to_string()
    //   )
    // }

    // {접수번호}_{처리기관이름}_{업로드_파일명}
    pub fn make_filename(
        registration_number: &str,
        proc_org_name: &str,
        file_name: &str,
    ) -> String {
        let re_illegal_symbols = Regex::new("[\"\n \t()\'~]").unwrap();
        let re_retouch = Regex::new("_+").unwrap();

        format!(
            "{}_{}_{}",
            registration_number,
            proc_org_name.replace(" ", ""),
            re_retouch
                .replace_all(&re_illegal_symbols.replace_all(file_name.trim(), "_"), "_",)
                .to_string()
        )
    }

    // // {접수번호}_{처리기관이름}_{업로드_파일명}
    // pub fn make_filename_by_row(bill: &models::Bill, file: &DntcFile) -> String {
    //   let re_illegal_symbols = Regex::new("[\"\n \t()\'~]").unwrap();
    //   let re_retouch = Regex::new("_+").unwrap();

    //   format!(
    //     "{}_{}_{}",
    //     bill.registration_number,
    //     bill.proc_org_name.replace(" ", ""),
    //     re_retouch
    //       .replace_all(
    //         &re_illegal_symbols.replace_all(&file.uploadFileOrginlNm.trim(), "_"),
    //         "_",
    //       )
    //       .to_string()
    //   )
    // }

    // pub fn mkdir_by_bill(&self, bill: &models::Bill) -> std::io::Result<()> {
    //   let dir_path = format!("{}/{}", &self._local_path, &self.make_dirname(bill));
    //   create_dir(Path::new(&dir_path))?;
    //   let dir_path_public = format!("{}/{}", &self._local_path_public, &self.make_dirname(bill));
    //   create_dir(Path::new(&dir_path_public))?;
    //   Ok(())
    // }

    // pub fn mkdir_by_bill_row(&self, bill: &models::Bill) -> std::io::Result<()> {
    //   create_dir(Path::new(&self.make_dirname_by_row(bill)))?;
    //   Ok(())
    // }

    pub fn save<T: Downloadable>(
        &self,
        downloaded_file: &Bytes,
        downloadable_bill: &T,
        orig_file_name: &str,
    ) -> Result<File, Box<dyn std::error::Error>> {
        let dir_path = format!("{}/{}", &self._local_path, downloadable_bill.get_dirname(),);
        let file_path = format!(
            "{}/{}",
            &dir_path,
            downloadable_bill.get_filename(orig_file_name)
        );

        match create_dir(Path::new(&dir_path)) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("이미 다운로드 받은 청구건입니다: {}\n", &file_path);
            }
        };

        let mut local_file = File::create(&file_path)?;
        io::copy(&mut downloaded_file.as_ref(), &mut local_file)?;
        Ok(local_file)
    }

    // pub fn save_by_row(&self, bill: &models::Bill, file: &DntcFile, downloaded_file: Bytes) -> File {
    //   let file_name = FileManager::make_filename_by_row(bill, file);
    //   let file_path = format!("{}/{}", self.make_dirname_by_row(bill), file_name);
    //   let mut local_file = File::create(file_path).unwrap();
    //   io::copy(&mut downloaded_file.as_ref(), &mut local_file).unwrap();
    //   local_file
    // }

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
            obj.into_commit()
                .map_err(|_| git2::Error::from_str("Couldn't find commit"))
        }

        let repo = match Repository::open(&self._local_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        let mut index = repo.index().unwrap();
        index
            .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
            .unwrap();
        let oid = index.write_tree().unwrap();
        let parent_commit = find_last_commit(&repo).unwrap();
        let tree = repo.find_tree(oid).unwrap();

        repo.commit(
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

pub trait Downloadable {
    fn get_filename(&self, orig_file_name: &str) -> String;
    fn get_dirname(&self) -> String;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_new() {
        let fm = FileManager::new("/Users/hoonyland/.ogk/.data", "hoonyland/data-opengirok");

        assert_eq!(true, false)
    }
}

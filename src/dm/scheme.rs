table! {
  bills (id) {
      id -> VarChar,
      group_id -> VarChar,
      proc_org_code -> VarChar,
      proc_org_name -> VarChar,
      proc_registration_number -> VarChar,
      public_date -> Nullable<VarChar>,
      registration_number -> VarChar,
      request_date -> VarChar,
      request_description -> Text,
      request_proc_registration_number -> VarChar,
      request_subject -> Text,
      result_description -> Nullable<Text>,
      status -> VarChar,
      user_id -> VarChar,
  }
}

table! {
  files (id) {
    id -> Integer,
    filename -> VarChar,
    bill_id -> VarChar,
  }
}

table! {
  users (username) {
      embago_month -> Nullable<Integer>,
      username -> VarChar,
      password -> VarChar,
  }
}

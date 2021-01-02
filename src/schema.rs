table! {
    bills (id) {
        id -> Varchar,
        group_id -> Varchar,
        proc_registration_number -> Varchar,
        proc_org_code -> Varchar,
        proc_org_name -> Varchar,
        public_date -> Varchar,
        registration_number -> Varchar,
        request_date -> Varchar,
        request_description -> Nullable<Text>,
        request_proc_registration_number -> Varchar,
        request_subject -> Text,
        status -> Varchar,
        user_id -> Varchar,
    }
}

table! {
    files (id) {
        id -> Integer,
        file_name -> Varchar,
        bill_id -> Varchar,
    }
}

table! {
    users (username) {
        username -> Varchar,
        password -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(bills, files, users,);

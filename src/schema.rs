table! {
    bills (id) {
        id -> Varchar,
        group_id -> Varchar,
        registration_number -> Varchar,
        request_date -> Varchar,
        request_subject -> Varchar,
        request_description -> Varchar,
        result_description -> Nullable<Varchar>,
        user_id -> Varchar,
        open_date -> Nullable<Varchar>,
        open_date_reason -> Nullable<Varchar>,
        open_status -> Varchar,
        open_type -> Nullable<Varchar>,
        proc_date -> Varchar,
        proc_org_addr -> Varchar,
        proc_org_code -> Varchar,
        proc_org_name -> Varchar,
        proc_org_phone -> Varchar,
        proc_dept_name -> Varchar,
        proc_person_class -> Varchar,
        proc_person_email -> Varchar,
    }
}

table! {
    files (id) {
        id -> Integer,
        filename -> Varchar,
        bill_id -> Varchar,
    }
}

table! {
    users (username) {
        username -> Varchar,
        password -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    bills,
    files,
    users,
);

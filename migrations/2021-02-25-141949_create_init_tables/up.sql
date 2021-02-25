CREATE TABLE bills (
  id VARCHAR(255) NOT NULL PRIMARY KEY,
  group_id VARCHAR(255) NOT NULL,
  registration_number VARCHAR(255) NOT NULL,
  request_date VARCHAR(255) NOT NULL,
  request_subject VARCHAR(255) NOT NULL,
  request_description VARCHAR(255) NOT NULL,
  result_description VARCHAR(255),
  user_id VARCHAR(255) NOT NULL,
  open_date VARCHAR(255),
  open_date_reason VARCHAR(255),
  open_status VARCHAR(255) NOT NULL,
  open_type VARCHAR(255),
  proc_date VARCHAR(255) NOT NULL,
  proc_org_addr VARCHAR(255) NOT NULL,
  proc_org_code VARCHAR(255) NOT NULL,
  proc_org_name VARCHAR(255) NOT NULL,
  proc_org_phone VARCHAR(255) NOT NULL,
  proc_dept_name VARCHAR(255) NOT NULL,
  proc_person_class VARCHAR(255) NOT NULL,
  proc_person_email VARCHAR(255) NOT NULL
);

CREATE TABLE files (
  id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
  filename VARCHAR(255) NOT NULL,
  bill_id VARCHAR(255) NOT NULL
);

CREATE TABLE users (
  username VARCHAR(255) NOT NULL PRIMARY KEY,
  password VARCHAR(255)
);
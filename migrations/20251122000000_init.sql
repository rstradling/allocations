create table if not exists roles
(
  id uuid default uuidv7() primary key,
  name text not null,
  scopes jsonb not null,
  created_at timestamp default CURRENT_TIMESTAMP,
  updated_at timestamp default CURRENT_TIMESTAMP
);

create table if not exists users
(
  id uuid default uuidv7() primary key,
  first_name text not null,
  last_name text not null,
  email text not null,
  external_id text not null,
  external_id_source text not null,
  role_id uuid references roles(id) on delete set null,
  created_at timestamp default CURRENT_TIMESTAMP,
  updated_at timestamp default CURRENT_TIMESTAMP,

  CONSTRAINT uk_users_email UNIQUE (email)

);

create table if not exists employees 
(
  id uuid default uuidv7() primary key,
  first_name text not null,
  last_name text not null,
  email text not null,
  salary numeric(15, 2) not null,

  CONSTRAINT uk_employees_email UNIQUE (email)
);

create table if not exists tags
(
  id uuid default uuidv7() primary key,
  tag text not null
);

create table if not exists initiatives
(
  id uuid default uuidv7() primary key,
  initiative text not null
);


create table if not exists assignments
(
  id uuid default uuidv7() primary key,
  year integer not null,
  jan numeric(3, 2) null,
  feb numeric(3, 2) null,
  mar numeric(3, 2) null,
  apr numeric(3, 2) null,
  may numeric(3, 2) null,
  jun numeric(3, 2) null,
  jul numeric(3, 2) null,
  aug numeric(3, 2) null,
  sep numeric(3, 2) null,
  oct numeric(3, 2) null,
  nov numeric(3, 2) null,
  "dec" numeric(3, 2) null
);

CREATE table if not exists employee_assignments
(
  id uuid DEFAULT uuidv7() PRIMARY KEY,
  employee_id uuid NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
  assignment_id uuid NOT NULL REFERENCES assignments(id) ON DELETE CASCADE,
  allocation numeric(3, 2) NOT NULL CHECK (allocation >= 0 AND allocation <= 1),
  initiative_id uuid NOT NULL REFERENCES initiatives(id) ON DELETE CASCADE,
  
  -- Ensure a person can't be associated with the same initiative more than once
  UNIQUE (employee_id, initiative_id),
  
  -- Ensure allocation is valid
  CHECK (allocation >= 0 AND allocation <= 1)
);

-- Junction table to connect assignments to tags
create table if not exists assignment_tags
(
  id uuid DEFAULT uuidv7() PRIMARY KEY,
  assignment_id uuid NOT NULL REFERENCES assignments(id) ON DELETE CASCADE,
  tag_id uuid NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  
  -- Ensure unique combination of assignment and tag
  UNIQUE (assignment_id, tag_id)
);

-- Junction table to connect employee assignments to tags
create table if not exists employee_assignment_tags
(
  id uuid DEFAULT uuidv7() PRIMARY KEY,
  employee_assignment_id uuid NOT NULL REFERENCES employee_assignments(id) ON DELETE CASCADE,
  tag_id uuid NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  
  -- Ensure unique combination of employee assignment and tag
  UNIQUE (employee_assignment_id, tag_id)
);


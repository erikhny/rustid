create table users(
    id uuid not null,
    primary key (id),
    email text not null unique,
    name text not null,
    created_at timestamptz not null
);
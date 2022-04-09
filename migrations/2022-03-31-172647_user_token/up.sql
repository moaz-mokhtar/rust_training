-- Your SQL goes here
-- Your SQL goes here
create table user_token(
    user_id uuid primary key references users (id) on delete cascade on update cascade,
    token varchar not null,
    created_at timestamp not null,
    expires_at timestamp not null
)
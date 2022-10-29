-- Add migration script here
alter table subscriptions
rename column subscribted_at to subscribed_at;
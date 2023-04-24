// @generated automatically by Diesel CLI.

diesel::table! {
    applications (id) {
        id -> Uuid,
        application_type -> Text,
        storage_id -> Nullable<Uuid>,
        db_tenant_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        role_type -> Text,
        user_id -> Nullable<Uuid>,
        application_id -> Nullable<Uuid>,
        db_tenant_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    storages (id) {
        id -> Uuid,
        db_tenant_id -> Nullable<Uuid>,
        storage_type -> Text,
        path -> Nullable<Text>,
        connection_string -> Nullable<Text>,
        schema -> Nullable<Text>,
        table_prefix -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tenants (id) {
        id -> Uuid,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        email_verified -> Bool,
        password -> Text,
        full_name -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        db_tenant_id -> Nullable<Uuid>,
    }
}

diesel::joinable!(applications -> storages (storage_id));
diesel::joinable!(applications -> tenants (db_tenant_id));
diesel::joinable!(roles -> applications (application_id));
diesel::joinable!(roles -> tenants (db_tenant_id));
diesel::joinable!(roles -> users (user_id));
diesel::joinable!(storages -> tenants (db_tenant_id));
diesel::joinable!(users -> tenants (db_tenant_id));

diesel::allow_tables_to_appear_in_same_query!(
    applications,
    roles,
    storages,
    tenants,
    users,
);

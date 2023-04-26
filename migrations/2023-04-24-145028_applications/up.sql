-- Your SQL goes here

CREATE TABLE "storages" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    db_tenant_id UUID references tenants(id),
    storage_type TEXT NOT NULL,
    path TEXT NULL,
    connection_string TEXT NULL,
    schema TEXT NULL,
    table_prefix TEXT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);


CREATE TABLE "applications" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    application_type TEXT NOT NULL,
    storage_id UUID references storages(id),
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP,
    db_tenant_id UUID references tenants(id)
);


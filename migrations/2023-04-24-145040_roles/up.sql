-- Your SQL goes here

CREATE TABLE "roles" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    role_type TEXT NOT NULL,
    user_id UUID references users(id),
    application_id UUID references applications(id),
    db_tenant_id UUID references tenants(id),
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);


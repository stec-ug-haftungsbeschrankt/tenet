pub mod dbuser;
pub mod dbtenant;
pub mod dbapplication;
pub mod dbrole;
pub mod dbstorage;
pub mod database;
pub mod service_error;

/*
use crate::postgresql::usertenants::user_tenant_service::user_tenant_service_server::UserTenantService;
use dbuser::DbUserMessage;
use dbtenant::DbTenantMessage;
use dbtenant::DbTenant;
use dbuser::DbUser;
use user_tenant_service::{Tenant, TenantRequest, TenantReply, TenantList, UserRequest, UserReply, UserList, User, Empty, AuthenticationStatus, AuthorizationStatus, Credentials, Roles};

use uuid::Uuid;
use tonic::{Request, Response, Status};




pub mod user_tenant_service {
    tonic::include_proto!("usertenants");
}


#[derive(Debug, Default)]
pub struct Service {

}



impl From<&DbUser> for User {
    fn from(user: &DbUser) -> Self {
        let tenant_id = match user.db_tenant_id {
            Some(t) => t.to_string(),
            _ => String::from("")
        };

        User {
            id: user.id.to_string(),
            email: user.email.clone(),
            email_verified: user.email_verified,
            password: user.password.clone(),
            full_name: user.full_name.clone(),
            tenant_id
        }
    }
}

impl From<&DbTenant> for Tenant {
    fn from(tenant: &DbTenant) -> Self {
        Tenant {
            id: tenant.id.to_string(),
            title: tenant.title.clone()
        }
    }
}


impl From<&Tenant> for DbTenantMessage {
    fn from(tenant: &Tenant) -> Self {
        DbTenantMessage {
            title: tenant.title.clone()
        }
    }
}

impl From<&User> for DbUserMessage {
    fn from(user: &User) -> Self {
        let tenant_id = match Uuid::parse_str(&user.tenant_id) {
            Ok(id) => Some(id),
            _ => None
        };

        DbUserMessage {    
            email: user.email.clone(),
            email_verified: user.email_verified,
            password: user.password.clone(),
            full_name: user.full_name.clone(),
            db_tenant_id: tenant_id
        }
    }
}




#[tonic::async_trait]
impl UserTenantService for Service {

    async fn get_all_tenants(&self, _request: Request<Empty>) -> Result<Response<TenantList>, Status> {
        let db_tenants = DbTenant::find_all()
            .map_err(|e| Status::unavailable(e.to_string()))?;
        
        let reply = TenantList {
            tenants: db_tenants.iter().map(|t| Tenant::from(t)).collect::<Vec<Tenant>>()
        };
            
        Ok(Response::new(reply))
    }


    async fn get_tenant(&self, request: Request<TenantRequest>) -> Result<Response<Tenant>, Status> {
        let request_id = request.into_inner().id;
        let id = Uuid::parse_str(&request_id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        
        let tenant = DbTenant::find(id)
            .map_err(|e| Status::not_found(e.to_string()))?;

        let reply = Tenant::from(&tenant);
        Ok(Response::new(reply))
    }


    async fn get_tenant_by_user(&self, request: Request<User>) -> Result<Response<Tenant>, Status> {
        let user = request.into_inner();
        let id = Uuid::parse_str(&user.tenant_id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let tenant = DbTenant::find(id)
            .map_err(|e| Status::not_found(e.to_string()))?;

        let reply = Tenant::from(&tenant);
        Ok(Response::new(reply))
    }


    async fn create_tenant(&self, request: Request<Tenant>) -> Result<Response<Tenant>, Status> {
        let tenant_request = request.into_inner(); 
        let message = DbTenantMessage::from(&tenant_request);
        let tenant = DbTenant::create(message)
            .map_err(|e| Status::already_exists(e.to_string()))?;
        
        let reply = Tenant::from(&tenant);
        Ok(Response::new(reply))
    }


    async fn update_tenant(&self, request: Request<Tenant>) -> Result<Response<Tenant>, Status> {
        let tenant_request = request.into_inner(); 
        let id = Uuid::parse_str(&tenant_request.id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let message = DbTenantMessage::from(&tenant_request);

        let updated_tenant = DbTenant::update(id, message)
            .map_err(|e| Status::aborted(e.to_string()))?;

        let reply = Tenant::from(&updated_tenant);
        Ok(Response::new(reply))
    }


    async fn delete_tenant(&self, request: Request<TenantRequest>) -> Result<Response<TenantReply>, Status> {
        let request_id = request.into_inner().id;
        let id = Uuid::parse_str(&request_id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        
        let _result = DbTenant::delete(id)
            .map_err(|e| Status::aborted(e.to_string()))?;
        let reply = TenantReply {
            id: request_id,
            success: true
        };
        Ok(Response::new(reply))
    }




    async fn get_all_users(&self, _request: Request<Empty>) -> Result<Response<UserList>, Status> {
        let db_users = DbUser::find_all()
            .map_err(|e| Status::unavailable(e.to_string()))?;

        let reply = UserList {
            users: db_users.iter().map(|u| User::from(u)).collect::<Vec<User>>()
        };
        Ok(Response::new(reply))
    }

    async fn get_users_by_tenant(&self, request: Request<Tenant>) -> Result<Response<UserList>, Status> {
        let tenant = request.into_inner();
        let tenant_id = Uuid::parse_str(&tenant.id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let db_users = DbUser::find_by_tenant(tenant_id)
            .map_err(|e| Status::unavailable(e.to_string()))?;

        let reply = UserList {
            users: db_users.iter().map(|u| User::from(u)).collect::<Vec<User>>()
        };
        Ok(Response::new(reply))
    }


    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<User>, Status> {
        let request_id = request.into_inner().id;
        let id = Uuid::parse_str(&request_id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        
        let user = DbUser::find(id)
            .map_err(|e| Status::not_found(e.to_string()))?;

        let reply = User::from(&user);
        Ok(Response::new(reply))
    }
    

    async fn create_user(&self, request: Request<User>) -> Result<Response<User>, Status> {
        let user_request = request.into_inner(); 
        let message = DbUserMessage::from(&user_request);
        let user = DbUser::create(message)
            .map_err(|e| Status::already_exists(e.to_string()))?;
        
        let reply = User::from(&user);
        Ok(Response::new(reply))
    }


    async fn update_user(&self, request: Request<User>) -> Result<Response<User>, Status> {
        let user_request = request.into_inner(); 
        let id = Uuid::parse_str(&user_request.id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let message = DbUserMessage::from(&user_request);

        let updated_user = DbUser::update(id, message)
            .map_err(|e| Status::aborted(e.to_string()))?;

        let reply = User::from(&updated_user);
        Ok(Response::new(reply))
    }


    async fn delete_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        let request_id = request.into_inner().id;
        let id = Uuid::parse_str(&request_id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        
        let _result = DbUser::delete(id)
            .map_err(|e| Status::aborted(e.to_string()))?;
        let reply = UserReply {
            id: request_id,
            success: true
        };
        Ok(Response::new(reply))
    }

    async fn is_authenticated(&self, request: Request<UserRequest>) -> Result<Response<AuthenticationStatus>, Status> {
        let request_id = request.into_inner().id;
        let db_user = DbUser::find(Uuid::parse_str(&request_id).unwrap())
            .map_err(|e| Status::not_found(e.to_string()))?;

        // FIXME Check if last authentication is less than 30 minutes old
        
        let reply = AuthenticationStatus {
            status: true
        };
        Ok(Response::new(reply))
    }

    async fn is_authorized(&self, request: Request<UserRequest>) -> Result<Response<AuthorizationStatus>, Status> {
        let reply = AuthorizationStatus {
            role: Roles::Administrator as i32
        };
        // FIXME 
        Ok(Response::new(reply))
    }


    async fn authenticate(&self, request: Request<Credentials>) -> Result<Response<UserReply>, Status> {
        let user_request = request.into_inner();

        let db_user = DbUser::find_by_email(user_request.email)
            .map_err(|e| Status::not_found(e.to_string()))?;

        let result = db_user.verify_password(&user_request.password);

        let reply = if result.is_ok() {
            UserReply {
                id: db_user.id.to_string(),
                success: true
            }
        } else {
            UserReply {
                id: db_user.id.to_string(),
                success: false
            }
        };

        Ok(Response::new(reply))
    }
}
*/
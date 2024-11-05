use ehttp::Request;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

#[allow(dead_code)]
struct Cedarling {
    authz: Authz,
    jwt: Jwt,
}

impl Cedarling {
    fn new(authz_conf: AuthzConfig, jwt_conf: JwtConfig) -> Self {
        let jwt = Jwt::new(jwt_conf);
        let authz = Authz::new(authz_conf);
        Self { authz, jwt }
    }

    fn authz(&self) {
        self.jwt.validate_tokens();
    }

    fn print_jwks(&self) {
        let jwks = self.jwt.jwks.read().unwrap();
        println!("{:?}", jwks);
    }
}

struct Authz(AuthzConfig);
struct AuthzConfig;

impl Authz {
    fn new(config: AuthzConfig) -> Self {
        Self(config)
    }
}

#[allow(dead_code)]
struct Jwt {
    config: JwtConfig,
    pub jwks: Arc<RwLock<Option<serde_json::Value>>>,
}
struct JwtConfig;

impl Jwt {
    fn new(config: JwtConfig) -> Self {
        Self {
            config,
            jwks: Arc::new(RwLock::new(None)),
        }
    }

    fn validate_tokens(&self) {
        let jwks = self.jwks.clone();
        let request = Request::get("https://test-casa.gluu.info/jans-auth/restv1/jwks");

        ehttp::fetch(request, move |result| match result {
            Ok(resp) => {
                // update local jwks
                let jwks_resp: serde_json::Value = serde_json::from_slice(&resp.bytes).unwrap();
                let mut jwks = jwks.write().unwrap();
                *jwks = Some(jwks_resp);
            }
            Err(_) => todo!(),
        });
    }
}

fn main() {
    let cedarling = Cedarling::new(AuthzConfig, JwtConfig);
    println!("cedarling initialized");

    cedarling.authz(); // authz will probably fail since jwks hasn't been updated yet
                       // what if i don't want authz to fail here?
    println!("authz done");

    cedarling.print_jwks(); // jwks will not be here
    thread::sleep(Duration::from_secs(2));

    cedarling.print_jwks(); // jwks will finially be
}

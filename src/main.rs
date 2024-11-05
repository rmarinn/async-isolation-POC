use ehttp::Request;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
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
        if !self.wait_for_jwks(Duration::from_secs(5)) {
            eprintln!("JWKS not available. Authorization may fail.");
        }
        self.authz.perform_authz();
    }

    fn wait_for_jwks(&self, timeout: Duration) -> bool {
        let start = Instant::now();
        while start.elapsed() < timeout {
            {
                let jwks = self.jwt.jwks.read().unwrap();
                if jwks.is_some() {
                    return true; // JWKS is available
                }
            }
            thread::sleep(Duration::from_millis(100)); // Poll every 100ms
        }
        false // JWKS not available within timeout
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

    fn perform_authz(&self) {
        // Implementation of your authorization logic goes here
        println!("Performing authorization...");
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

    cedarling.authz();
    println!("authz done");
    cedarling.print_jwks(); // jwks should be here now
}

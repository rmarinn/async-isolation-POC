use cedar_policy::{Authorizer, Context, Entities, PolicySet, Request};
use std::sync::{Arc, RwLock};
use tokio::time::{sleep, Duration};

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

    async fn authz(&self) {
        self.jwt.validate_tokens().await;
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
    jwks: Arc<RwLock<Option<String>>>,
}
struct JwtConfig;

impl Jwt {
    fn new(config: JwtConfig) -> Self {
        Self {
            config,
            jwks: Arc::new(RwLock::new(None)),
        }
    }

    async fn validate_tokens(&self) {
        // simulate network delay
        println!("sending http request");
        sleep(Duration::from_millis(1000)).await;
        println!("received http response");

        // update local jwks
        let mut jwks = self.jwks.write().expect("failed to obtain write lock");
        *jwks = Some("jwks".to_string());
    }
}

fn run_cedar() {
    println!("Testing Cedar:");

    const POLICY_SRC: &str = r#"
permit(principal == User::"alice", action == Action::"view", resource == File::"93");
"#;
    let policy: PolicySet = POLICY_SRC.parse().unwrap();

    let action = r#"Action::"view""#.parse().unwrap();

    let alice = r#"User::"alice""#.parse().unwrap();
    let file = r#"File::"93""#.parse().unwrap();
    let request = Request::new(alice, action, file, Context::empty(), None).unwrap();

    let entities = Entities::empty();
    let authorizer = Authorizer::new();
    let answer = authorizer.is_authorized(&request, &policy, &entities);

    // Should output `Allow`
    println!("{:?}", answer.decision());

    let action = r#"Action::"view""#.parse().unwrap();
    let bob = r#"User::"bob""#.parse().unwrap();
    let file = r#"File::"93""#.parse().unwrap();
    let request = Request::new(bob, action, file, Context::empty(), None).unwrap();

    let answer = authorizer.is_authorized(&request, &policy, &entities);

    // Should output `Deny`
    println!("{:?}", answer.decision());
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cedarling = Cedarling::new(AuthzConfig, JwtConfig);
    println!("cedarling initialized");
    cedarling.authz().await;
    println!("authz done");
    run_cedar();
}

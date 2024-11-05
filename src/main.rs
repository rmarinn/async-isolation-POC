use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, RwLock};
use threadpool::ThreadPool;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
struct Cedarling {
    authz: Authz,
    jwt: Jwt,
}

impl Cedarling {
    fn new(authz_conf: AuthzConfig, jwt_conf: JwtConfig) -> Self {
        let jwt_rx = Jwt::new(jwt_conf);
        println!("started jwt init");

        let authz = Authz::new(authz_conf);
        println!("authz initialized");

        let jwt = jwt_rx.recv().expect("failed to receive JwtService");
        Self { authz, jwt }
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
    pool: ThreadPool,
    jwks: Arc<RwLock<Option<String>>>,
}
struct JwtConfig;

impl Jwt {
    fn new(config: JwtConfig) -> Receiver<Self> {
        let (tx, rx) = mpsc::channel();

        let pool = ThreadPool::new(4);
        let jwks = Arc::new(RwLock::new(None));
        pool.clone().execute(move || {
            let runtime = Runtime::new().unwrap();

            // asnyc calls get isolated here
            runtime.block_on(async {
                fetch_jwks(jwks.clone()).await;
            });

            tx.send(Self { config, pool, jwks })
                .expect("error sending result");
        });

        rx
    }
}

// simulate network request
async fn fetch_jwks(jwks: Arc<RwLock<Option<String>>>) {
    // simulate network delay
    println!("sending http request");
    sleep(Duration::from_millis(1000)).await;
    println!("received http response");

    // update local jwks
    let mut jwks_lock = jwks.write().expect("failed to obtain write lock");
    *jwks_lock = Some("jwks".to_string());
}

fn main() {
    let _cedarling = Cedarling::new(AuthzConfig, JwtConfig);
    println!("cedarling initialized")
}

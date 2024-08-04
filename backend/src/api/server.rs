use axum::Router;
use colorful::Colorful;
use socketioxide::extract::SocketRef;
use socketioxide::SocketIo;
use tower_http::cors::CorsLayer;

use crate::{tui_success, tui_warn};
use crate::api::AppState;
use crate::api::rest::build_rest_routes;
use crate::api::sockets::build_socket_routes;
use crate::utils::config::Config;
use crate::utils::database::Database;

/// Server (both REST and SocketIO).
pub struct Server {
    config: Config,
    custom_routers: Vec<Router<AppState>>,
    custom_sockets: Vec<fn(socket: &SocketRef)>,
}

impl Server {
    pub fn from(config: Config) -> Self {
        Self {
            config,
            custom_routers: vec![],
            custom_sockets: vec![],
        }
    }

    /// Adds custom routes to the server.
    pub fn with_custom_routes(self, custom_router: Router<AppState>) -> Self {
        let mut custom_routers = self.custom_routers;
        custom_routers.push(custom_router);

        Server {
            config: self.config,
            custom_routers,
            custom_sockets: self.custom_sockets,
        }
    }

    /// Adds custom sockets events to the server.
    pub fn with_custom_socket_events(self, register_events: fn(socket: &SocketRef)) -> Self {
        let mut custom_sockets = self.custom_sockets;
        custom_sockets.push(register_events);
        Server {
            config: self.config,
            custom_routers: self.custom_routers,
            custom_sockets,
        }
    }

    /// Starts the server.
    pub async fn start(self) -> anyhow::Result<()> {
        // Build the database.
        let path = self.config.db_folder;
        let database = match Database::init_persistent(path.clone(), false, true) {
            Ok(database) => {
                tui_success!("Database ready - saving to", path.to_str().unwrap());
                database
            }
            Err(err) => {
                tui_warn!(
                    "No persistent storage created - all information will be lost",
                    err.to_string().split('\n').next().unwrap()
                );
                Database::init_volatile().unwrap()
            }
        };

        // Build the socket API server.
        let (socket_layer, socket) = build_socket_routes(self.custom_sockets);

        // Build the REST API server.
        let mut routes = build_rest_routes();
        for custom_router in self.custom_routers {
            routes = routes.merge(custom_router);
        }

        let app = Router::from(routes)
            .layer(socket_layer)
            .layer(CorsLayer::permissive())
            .with_state(AppState { database, socket });

        let listener = tokio::net::TcpListener::bind((self.config.host, self.config.port)).await?;
        tui_success!(
            "Server ready at",
            match self.config.host.is_loopback() || self.config.host.is_unspecified() {
                true => format!("http://localhost:{}", self.config.port),
                false => format!("http://{}", listener.local_addr()?.to_string()),
            }
        );
        tui_success!("Application is now started (press Ctrl+C to stop gracefully)");

        axum::serve(listener, app)
            .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
            .await?;

        Ok(())
    }
}

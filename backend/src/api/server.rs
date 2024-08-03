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
//
// /// Add conversion with serde_json::Value for DeviceState.
// impl From<serde_json::Value> for DeviceState {
//     fn from(value: serde_json::Value) -> Self {
//         match value {
//             serde_json::Value::Null => DeviceState::Null,
//             serde_json::Value::Bool(b) => DeviceState::Boolean(b),
//             serde_json::Value::Number(n) => {
//                 if let Some(u) = n.as_u64() {
//                     DeviceState::Integer(u)
//                 } else if let Some(i) = n.as_i64() {
//                     DeviceState::Signed(i)
//                 } else if let Some(f) = n.as_f64() {
//                     DeviceState::Float(f)
//                 } else {
//                     DeviceState::Integer(0)
//                 }
//             }
//             serde_json::Value::String(s) => DeviceState::String(s),
//             serde_json::Value::Array(list) => {
//                 DeviceState::Array(list.into_iter().map(Into::into).collect())
//             }
//             serde_json::Value::Object(map) => DeviceState::Object(
//                 map.into_iter()
//                     .map(|(key, value)| (key, value.into()))
//                     .collect(),
//             ),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use axum::body;
//     use axum::body::Body;
//     use axum::extract::Request;
//     use axum::http::{header, Method, StatusCode, Uri};
//     use hyper_util::client::legacy::Client;
//     use std::time::Duration;
//
//     use tokio::time::sleep;
//
//     use crate::api::{ApiServerTask, RejectionError};
//     use crate::utils::tasks::Task;
//
//     #[tokio::test]
//     async fn test_server_start() {
//         let client = Client::new();
//
//         // Start and wait until server is ready.
//         let handler = tokio::spawn(async { ApiServerTask::default().start().await });
//         sleep(Duration::from_millis(500)).await;
//
//         // Redirection to v1
//         let res = client
//             .get(Uri::from_static("http://127.0.0.1:3030"))
//             .await
//             .unwrap();
//         assert_eq!(res.status(), StatusCode::MOVED_PERMANENTLY);
//         let loc = res
//             .headers()
//             .get(header::LOCATION)
//             .unwrap()
//             .to_str()
//             .unwrap();
//         assert_eq!(loc, "/v1/");
//
//         // --------
//         // Rejection transformed to errors.
//
//         // NOT FOUND.
//         let res = client
//             .get(Uri::from_static("http://127.0.0.1:3030/v1/unfound"))
//             .await
//             .unwrap();
//         assert_eq!(res.status(), StatusCode::NOT_FOUND);
//         let body = String::from_utf8(
//             body::to_bytes(res.into_body(), usize::MAX)
//                 .await
//                 .unwrap()
//                 .to_vec(),
//         )
//         .unwrap();
//         assert_eq!(
//             body,
//             serde_json::to_string(&RejectionError {
//                 code: StatusCode::NOT_FOUND,
//                 id: "NOT_FOUND".into(),
//                 message: "The request URL is not found.".into(),
//                 explanation: "The resource you are trying to access does not exist.".into(),
//                 action: "Please refer to API documentation to find out the available endpoints."
//                     .into(),
//             })
//             .unwrap(),
//             "Response body should contain NOT_FOUND rejection error"
//         );
//
//         // UNEXPECTED METHOD.
//         let req = Request::builder()
//             .method(Method::POST)
//             .uri("http://127.0.0.1:3030/v1")
//             .body(Body::from("foobar"))
//             .expect("request builder");
//         let res = client.request(req).await.unwrap();
//         assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
//         let body = String::from_utf8(
//             body::to_bytes(res.into_body(), usize::MAX)
//                 .await
//                 .unwrap()
//                 .to_vec(),
//         )
//         .unwrap();
//         assert_eq!(body, serde_json::to_string(&RejectionError {
//             code: StatusCode::METHOD_NOT_ALLOWED,
//             id: "METHOD_NOT_ALLOWED".into(),
//             message: "The request URL exists but the wrong method is applied.".into(),
//             explanation: "Some endpoints do not support all GET, POST, PUT, PATCH, DELETE request methods.".into(),
//             action: "Please refer to API documentation to find out the available endpoints.".into(),
//         }).unwrap(), "Response body should contain NOT_FOUND rejection error");
//
//         handler.abort()
//     }
// }

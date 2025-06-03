use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Deserialize, ToSchema)]
struct OperacionMagicaParams {
    primer_numero_magico: f64,
    segundo_numero_magico: f64,
}

#[derive(Serialize, ToSchema)]
struct ResultadoMagico {
    resultado: f64,
}

#[derive(Serialize, ToSchema)]
struct ErrorMagico {
    error: String,
}

#[utoipa::path(
    get,
    path = "/operacion_magica",
    params(
        ("primer_numero_magico" = f64, Query, description = "Primer número mágico"),
        ("segundo_numero_magico" = f64, Query, description = "Segundo número mágico")
    ),
    responses(
        (status = 200, description = "Operación mágica exitosa", body = ResultadoMagico),
        (status = 400, description = "Error en la operación mágica", body = ErrorMagico)
    )
)]
async fn operacion_magica(Query(params): Query<OperacionMagicaParams>) -> impl IntoResponse {
    if params.segundo_numero_magico == 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorMagico {
                error: "No se puede dividir por cero!".to_string(),
            }),
        )
            .into_response();
    }

    let resultado = params.primer_numero_magico / params.segundo_numero_magico;
    (StatusCode::OK, Json(ResultadoMagico { resultado })).into_response()
}

#[derive(OpenApi)]
#[openapi(
    paths(operacion_magica),
    components(schemas(ResultadoMagico, OperacionMagicaParams, ErrorMagico)),
    tags(
        (name = "Server de Magia", description = "Endpoints de uso de magia")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/operacion_magica", get(operacion_magica))
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Servidor corriendo en http://{}/docs", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

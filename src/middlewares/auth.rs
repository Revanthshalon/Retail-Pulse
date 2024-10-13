use crate::utils::verify_token;
use crate::AppState;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;

pub async fn jwt_auth_middleware(
    State(app_state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Response {
    let token = bearer.token().to_string();

    let claims_result = verify_token(&app_state, &token);

    match claims_result {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            next.run(request).await
        }
        Err(e) => e.into_response(),
    }
}

use axum::body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!(".vue");

pub async fn embed_vue(Path(path): Path<String>) -> Result<Response, StatusCode> {
	let path = path.trim_start_matches('/');

	STATIC_DIR.get_file(path).map_or(Err(StatusCode::NOT_FOUND), |file| {
		Ok((
			[(axum::http::header::CONTENT_TYPE, axum::http::HeaderValue::from_static("application/json"))],
			file.contents(),
		)
			.into_response())
	})
}

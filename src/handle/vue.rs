use axum::body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use include_dir::{include_dir, Dir};

static ASSETS: Dir<'_> = include_dir!(".vue");

pub async fn index() -> impl IntoResponse {
	let path = "index.html";
	ASSETS.get_file(path.clone()).map_or(Err(StatusCode::NOT_FOUND), |file| {
		// 使用 mime_guess 和文件的扩展名来推测 MIME 类型
		let mime = mime_guess::from_path(&path).first_or_octet_stream();
		Ok(([(axum::http::header::CONTENT_TYPE, mime.as_ref())], file.contents()).into_response())
	})
}

pub async fn assets(Path(path): Path<String>) -> impl IntoResponse {
	let path = format!("assets/{}", path);
	ASSETS.get_file(path.clone()).map_or(Err(StatusCode::NOT_FOUND), |file| {
		// 使用 mime_guess 和文件的扩展名来推测 MIME 类型
		let mime = mime_guess::from_path(&path).first_or_octet_stream();
		Ok(([(axum::http::header::CONTENT_TYPE, mime.as_ref())], file.contents()).into_response())
	})
}

pub async fn favicon() -> impl IntoResponse {
	let path = "favicon.ico";
	ASSETS.get_file(path.clone()).map_or(Err(StatusCode::NOT_FOUND), |file| {
		// 使用 mime_guess 和文件的扩展名来推测 MIME 类型
		let mime = mime_guess::from_path(&path).first_or_octet_stream();
		Ok(([(axum::http::header::CONTENT_TYPE, mime.as_ref())], file.contents()).into_response())
	})
}

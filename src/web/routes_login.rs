use crate::{
	crypt::{pwd, EncryptContent},
	model::{
		user::{UserBmc, UserForLogin},
		ModelManager,
	},
	web::{self, Error, Result},
	Ctx,
};
use axum::{debug_handler, extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/login", post(api_login_handler))
		.with_state(mm)
}

#[debug_handler]
async fn api_login_handler(
	mm: State<ModelManager>,
	cookies: Cookies,
	Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
	let LoginPayload {
		username,
		pwd: pwd_clear,
	} = payload;

	let root_ctx = Ctx::root_ctx();

	// -- Get the user
	let user: UserForLogin =
		UserBmc::first_by_username(&root_ctx, &mm, &username)
			.await?
			.ok_or(Error::LoginFailUsernameNotFound)?;
	let user_id = user.id;

	let Some(pwd) = user.pwd else {
		return Err(Error::LoginFailUserHasNoPwd { user_id });
	};

	pwd::validate_pwd(
		&EncryptContent {
			salt: user.pwd_salt.to_string(),
			content: pwd_clear.clone(),
		},
		&pwd,
	)
	.map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

	// -- Set web token.
	web::set_token_cookie(
		&cookies,
		&user.username,
		&user.token_salt.to_string(),
	)?;

	// Create the success body.
	let body = Json(json!({"result": {"success":true}}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	pwd: String,
}

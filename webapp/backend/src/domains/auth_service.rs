use super::dto::auth::LoginResponseDto;
use crate::errors::AppError;
use crate::models::user::{Dispatcher, Session, User};
use crate::utils::{generate_session_token, hash_password, verify_password};
use actix_web::web::Bytes;
use log::error;
use std::path::{Path, PathBuf};
use tokio::process::Command; // tokioのCommandをインポート
pub trait AuthRepository {
    async fn create_user(&self, username: &str, password: &str, role: &str)
        -> Result<(), AppError>;
    async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, AppError>;
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn create_dispatcher(&self, user_id: i32, area_id: i32) -> Result<(), AppError>;
    async fn find_dispatcher_by_id(&self, id: i32) -> Result<Option<Dispatcher>, AppError>;
    async fn find_dispatcher_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<Dispatcher>, AppError>;
    async fn find_profile_image_name_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<String>, AppError>;
    async fn create_session(&self, user_id: i32, session_token: &str) -> Result<(), AppError>;
    async fn delete_session(&self, session_token: &str) -> Result<(), AppError>;
    async fn find_session_by_session_token(&self, session_token: &str)
        -> Result<Session, AppError>;
}
#[derive(Debug)]
pub struct AuthService<T: AuthRepository + std::fmt::Debug> {
    repository: T,
}
impl<T: AuthRepository + std::fmt::Debug> AuthService<T> {
    pub fn new(repository: T) -> Self {
        AuthService { repository }
    }
    fn create_login_response(
        user: &User,
        session_token: String,
        dispatcher: Option<&Dispatcher>, // 参照を取る
    ) -> LoginResponseDto {
        LoginResponseDto {
            user_id: user.id,
            username: user.username.clone(),
            session_token,
            role: user.role.clone(),
            dispatcher_id: dispatcher.map(|d| d.id),
            area_id: dispatcher.map(|d| d.area_id),
        }
    }
    pub async fn register_user(
        &self,
        username: &str,
        password: &str,
        role: &str,
        area: Option<i32>,
    ) -> Result<LoginResponseDto, AppError> {
        if role == "dispatcher" && area.is_none() {
            return Err(AppError::BadRequest);
        }
        if self
            .repository
            .find_user_by_username(username)
            .await?
            .is_some()
        {
            return Err(AppError::Conflict);
        }
        let hashed_password = hash_password(password).map_err(|_| AppError::InternalServerError)?;
        self.repository
            .create_user(username, &hashed_password, role)
            .await?;
        let session_token = generate_session_token();
        let user = self
            .repository
            .find_user_by_username(username)
            .await?
            .ok_or(AppError::InternalServerError)?;
        self.repository
            .create_session(user.id, &session_token)
            .await?;
        if user.role == "dispatcher" {
            let area_id = area.unwrap();
            self.repository.create_dispatcher(user.id, area_id).await?;
            let dispatcher = self
                .repository
                .find_dispatcher_by_user_id(user.id)
                .await?
                .ok_or(AppError::InternalServerError)?;
            Ok(Self::create_login_response(
                &user,
                session_token,
                Some(&dispatcher),
            )) // 参照を渡す
        } else {
            Ok(Self::create_login_response(&user, session_token, None))
        }
    }
    pub async fn login_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<LoginResponseDto, AppError> {
        let user = self
            .repository
            .find_user_by_username(username)
            .await?
            .ok_or(AppError::Unauthorized)?;
        if !verify_password(&user.password, password).unwrap() {
            return Err(AppError::Unauthorized);
        }
        let session_token = generate_session_token();
        let create_session_task = self.repository.create_session(user.id, &session_token);
        let (dispatcher_id, area_id) = if user.role == "dispatcher" {
            let dispatcher = self
                .repository
                .find_dispatcher_by_user_id(user.id)
                .await?
                .ok_or(AppError::InternalServerError)?;
            (Some(dispatcher.id), Some(dispatcher.area_id))
        } else {
            (None, None)
        };
        create_session_task.await?;
        Ok(LoginResponseDto {
            user_id: user.id,
            username: user.username.clone(),
            session_token,
            role: user.role.clone(),
            dispatcher_id,
            area_id,
        })
    }
    pub async fn logout_user(&self, session_token: &str) -> Result<(), AppError> {
        self.repository.delete_session(session_token).await?;
        Ok(())
    }
    pub async fn get_resized_profile_image_byte(
        &self,
        user_id: i32,
        width: i32,
        height: i32,
    ) -> Result<Bytes, AppError> {
        let profile_image_name = self
            .repository
            .find_profile_image_name_by_user_id(user_id)
            .await?
            .ok_or(AppError::NotFound)?;
        let path: PathBuf =
            Path::new(&format!("images/user_profile/{}", profile_image_name)).to_path_buf();
        // 非同期タスクを生成
        let resize_task = tokio::spawn(async move {
            Command::new("convert")
                .arg(&path)
                .arg("-resize")
                .arg(format!("{}x{}!", width, height))
                .arg("png:-")
                .output()
                .await
        });
        // タスクを待機して結果を取得
        let output = resize_task
            .await
            .map_err(|_| AppError::InternalServerError)?;
        // spawnでのエラーを処理
        let output = output.map_err(|e| {
            error!("コマンド実行中にエラーが発生しました: {:?}", e);
            AppError::InternalServerError
        })?;
        // 結果の処理
        if output.status.success() {
            Ok(Bytes::from(output.stdout))
        } else {
            error!(
                "画像リサイズのコマンド実行に失敗しました: {:?}",
                String::from_utf8_lossy(&output.stderr)
            );
            Err(AppError::InternalServerError)
        }
    }
    pub async fn validate_session(&self, session_token: &str) -> Result<bool, AppError> {
        let session = self
            .repository
            .find_session_by_session_token(session_token)
            .await?;
        Ok(session.is_valid)
    }
}

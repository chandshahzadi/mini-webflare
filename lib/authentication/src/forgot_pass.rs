use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::query;
use utils::db::DB;
use utils::error::AppError;

#[derive(Deserialize)]
pub struct ForgotPassword {
    pub email: String,
}

impl ForgotPassword {
    pub async fn request(self, db: DB) -> Result<(), AppError> {
        let Some(user) = query!("SELECT id FROM users where email = $1", self.email)
            .fetch_optional(&db)
            .await?
        else {
            return Ok(());
        };
        let raw_token = hex::encode(rand::random::<[u8; 32]>());
        let token_hash = hex::encode(Sha256::digest(&raw_token));

        query!("DELETE FROM password_resets WHERE user_id = $1", user.id)
            .execute(&db)
            .await?;

        query!(
            "INSERT INTO password_resets (user_id, token_hash, expires_at)
                VALUES ($1, $2, NOW() + INTERVAL '15 minutes')",
            user.id,
            token_hash
        )
        .execute(&db)
        .await?;

        async fn send_reset_email(
            to: &str,
            token: &str,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let link = format!(
                "{}/reset-password?token={token}",
                std::env::var("FRONTEND_URL")?
            );

            let email = Message::builder()
                .from(std::env::var("SMTP_FROM")?.parse()?)
                .to(to.parse()?)
                .subject("Reset your password")
                .body(format!(
                    "Click to reset your password (valid 15 min):\n{link}"
                ))?;

            let creds = Credentials::new(std::env::var("SMTP_USER")?, std::env::var("SMTP_PASS")?);
            let mailer =
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&std::env::var("SMTP_HOST")?)?
                    .credentials(creds)
                    .build();

            mailer.send(email).await?;
            Ok(())
        }
        tokio::spawn(async move {
            if let Err(e) = send_reset_email(&self.email, &raw_token).await {
                eprintln!("failed to send reset email: {e}");
            }
        });
        Ok(())
    }
}

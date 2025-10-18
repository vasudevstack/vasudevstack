use std::process::{Command, Stdio};
use std::time::Duration;
use thiserror::Error;
use tokio::time;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authentication failed")]
    Failed,
    #[error("Internal error: {0}")]
    Internal(String),
}

pub async fn authenticate(username: &str, password: &str) -> Result<bool, AuthError> {
    let helper_path = "/etc/vasudevstack/auth/auth-helper";

    println!("[DEBUG] Starting authentication for username: '{}'", username);
    
    let child = match Command::new(helper_path)
        .arg(username)
        .arg(password)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => {
            println!("[DEBUG] Spawned auth-helper process with PID: {}", c.id());
            c
        }
        Err(e) => {
            println!("[ERROR] Failed to spawn auth-helper: {}", e);
            return Err(AuthError::Internal(format!("Failed to spawn auth-helper: {}", e)));
        }
    };

    let timeout = Duration::from_secs(5);
    println!("[DEBUG] Waiting for auth-helper output with timeout: {}s", timeout.as_secs());

    let output_res = time::timeout(
        timeout,
        tokio::task::spawn_blocking(move || child.wait_with_output()),
    )
    .await;

    let output = match output_res {
        Ok(Ok(Ok(o))) => {
            println!("[DEBUG] Auth-helper finished successfully");
            o
        }
        Ok(Ok(Err(e))) => {
            println!("[ERROR] Failed to wait on auth-helper: {}", e);
            return Err(AuthError::Internal(format!("Failed to wait on auth-helper: {}", e)));
        }
        Ok(Err(e)) => {
            println!("[ERROR] Failed to join spawn_blocking task: {}", e);
            return Err(AuthError::Internal(format!(
                "Failed to join spawn_blocking task: {}",
                e
            )));
        }
        Err(_) => {
            println!("[ERROR] auth-helper timed out after {}s", timeout.as_secs());
            return Err(AuthError::Internal("auth-helper timed out".to_string()));
        }
    };
    
    let stderr_raw = String::from_utf8_lossy(&output.stderr);
    let stderr = stderr_raw.trim();
    let stdout_raw = String::from_utf8_lossy(&output.stdout);
    let stdout = stdout_raw
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim();

    println!("[DEBUG] auth-helper stdout: '{}'", stdout);
    println!("[DEBUG] auth-helper stderr: '{}'", stderr);

    match stdout {
        "OK" => {
            println!("[DEBUG] Authentication successful for username: '{}'", username);
            Ok(true)
        }
        "FAIL" => {
            println!("[DEBUG] Authentication failed (invalid credentials) for username: '{}'", username);
            Ok(false)
        }
        other => {
            let msg = if other.is_empty() { stderr } else { other };
            println!("[ERROR] Unexpected output from auth-helper: '{}'", msg);
            Err(AuthError::Internal(format!(
                "Unexpected output from auth-helper: {}",
                msg
            )))
        }
    }
}

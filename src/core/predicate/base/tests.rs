use std::{error::Error, fmt, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    core::context::Ref,
    types::{Integer, User},
};

use super::*;

#[tokio::test]
async fn decorator() {
    let condition = Ref::new(Condition::new());
    let handler = Predicate::new(has_access, process_user);
    let user_1 = create_user(1);
    let user_2 = create_user(2);
    let user_3 = create_user(3);

    assert!(matches!(
        handler.handle(((user_1.clone(),), (user_1, condition.clone()))).await,
        PredicateOutput::True(Ok(()))
    ));
    assert!(*condition.value.lock().await);
    condition.set(false).await;

    assert!(matches!(
        handler.handle(((user_2.clone(),), (user_2, condition.clone()))).await,
        PredicateOutput::False
    ));
    assert!(!*condition.value.lock().await);
    condition.set(false).await;

    assert!(matches!(
        handler.handle(((user_3.clone(),), (user_3, condition.clone()))).await,
        PredicateOutput::True(Err(_))
    ));
    assert!(*condition.value.lock().await);
    condition.set(false).await;
}

fn create_user(id: Integer) -> User {
    User::new(id, format!("test #{}", id), false)
}

async fn has_access(user: User) -> PredicateResult {
    if user.id != 2 {
        PredicateResult::True
    } else {
        PredicateResult::False
    }
}

async fn process_user(user: User, condition: Ref<Condition>) -> Result<(), ProcessError> {
    condition.set(true).await;
    log::info!("Processing user: {:?}", user);
    if user.id == 3 {
        Err(ProcessError)
    } else {
        Ok(())
    }
}

#[derive(Clone)]
struct Condition {
    value: Arc<Mutex<bool>>,
}

impl Condition {
    fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(false)),
        }
    }

    async fn set(&self, value: bool) {
        *self.value.lock().await = value;
    }
}

#[derive(Debug)]
struct ProcessError;

impl Error for ProcessError {}

impl fmt::Display for ProcessError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "Process error")
    }
}

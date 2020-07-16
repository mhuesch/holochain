#![allow(clippy::mutex_atomic)]
use super::*;
use ::fixt::prelude::*;
use futures::Future;
use std::{
    marker::PhantomData,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};
use tracing::*;

// TODO write tests to verify the invariant.
/// This is needed to use the database where
/// the lifetimes cannot be verified by
/// the compiler (e.g. with wasmer).
/// The checks are moved to runtime.
/// This type cannot write to the db.
/// It only takes a [Reader].
/// ## Thread Safety
/// This type is `Send` and `Sync`
/// It's best to imagine it like a regular `&` except that it
/// is enforced at runtime.
/// ### Mutex
/// A mutex is used to guarantee that no one else is reading or
/// writing to the data but this is never contested
/// because of the single threaded nature.
/// Default is used to avoid serde
#[derive(Debug, Clone, Default)]
pub struct UnsafeInvokeZomeWorkspace {
    workspace: std::sync::Weak<tokio::sync::RwLock<AtomicPtr<std::ffi::c_void>>>,
}

fixturator!(
    UnsafeInvokeZomeWorkspace;
    curve Empty {
        let fake_ptr = std::ptr::NonNull::<std::ffi::c_void>::dangling().as_ptr();
        let guard = Arc::new(tokio::sync::RwLock::new(AtomicPtr::new(fake_ptr)));
        let workspace = Arc::downgrade(&guard);
        // Make sure the weak Arc cannot be upgraded
        std::mem::drop(guard);
        UnsafeInvokeZomeWorkspace { workspace }
    };
    curve Unpredictable {
        UnsafeInvokeZomeWorkspaceFixturator::new(Empty)
            .next()
            .unwrap()
    };
    curve Predictable {
        UnsafeInvokeZomeWorkspaceFixturator::new(Empty)
            .next()
            .unwrap()
    };
);

// TODO: SAFETY: Tie the guard to the lmdb `'env` lifetime.
/// If this guard is dropped the underlying
/// ptr cannot be used.
/// ## Safety
/// Don't use `mem::forget` on this type as it will
/// break the checks.
pub struct UnsafeInvokeZomeWorkspaceGuard<'env> {
    workspace: Option<Arc<tokio::sync::RwLock<AtomicPtr<std::ffi::c_void>>>>,
    phantom: PhantomData<&'env ()>,
}

impl UnsafeInvokeZomeWorkspace {
    pub fn from_mut<'env>(
        workspace: &'env mut InvokeZomeWorkspace,
    ) -> (UnsafeInvokeZomeWorkspaceGuard<'env>, Self) {
        let raw_ptr = workspace as *mut InvokeZomeWorkspace as *mut std::ffi::c_void;
        let guard = Arc::new(tokio::sync::RwLock::new(AtomicPtr::new(raw_ptr)));
        let workspace = Arc::downgrade(&guard);
        let guard = UnsafeInvokeZomeWorkspaceGuard {
            workspace: Some(guard),
            phantom: PhantomData,
        };
        let workspace = Self { workspace };
        (guard, workspace)
    }

    /// Useful when we need this type where we don't want to use it.
    /// It will always return None.
    pub fn null() -> Self {
        let fake_ptr = std::ptr::NonNull::<std::ffi::c_void>::dangling().as_ptr();
        let guard = Arc::new(tokio::sync::RwLock::new(AtomicPtr::new(fake_ptr)));
        let workspace = Arc::downgrade(&guard);
        // Make sure the weak Arc cannot be upgraded
        std::mem::drop(guard);
        Self { workspace }
    }

    pub async unsafe fn apply_ref<
        'a,
        R,
        Fut: Future<Output = R> + 'a,
        F: FnOnce(&'a InvokeZomeWorkspace) -> Fut,
    >(
        &self,
        f: F,
    ) -> Result<R, error::UnsafeInvokeZomeWorkspaceError> {
        // Check it exists
        match self.workspace.upgrade() {
            // Check that no-one else can write
            Some(lock) => {
                let guard = lock.read().await;
                let s = {
                    let sc = guard.load(Ordering::SeqCst);
                    let sc = sc as *const InvokeZomeWorkspace;
                    match sc.as_ref() {
                        Some(s) => s,
                        None => Err(error::UnsafeInvokeZomeWorkspaceError::GuardDropped)?,
                    }
                };
                Ok(f(s).await)
            }
            None => Err(error::UnsafeInvokeZomeWorkspaceError::GuardDropped),
        }
    }

    pub async unsafe fn apply_mut<
        'a,
        R,
        Fut: Future<Output = R> + 'a,
        F: FnOnce(&'a mut InvokeZomeWorkspace) -> Fut,
    >(
        &self,
        f: F,
    ) -> Result<R, error::UnsafeInvokeZomeWorkspaceError> {
        // Check it exists
        match self.workspace.upgrade() {
            // Check that no-one else can write
            Some(lock) => {
                let guard = lock.write().await;
                let s = {
                    let sc = guard.load(Ordering::SeqCst);
                    let sc = sc as *mut InvokeZomeWorkspace;
                    match sc.as_mut() {
                        Some(s) => s,
                        None => Err(error::UnsafeInvokeZomeWorkspaceError::GuardDropped)?,
                    }
                };
                Ok(f(s).await)
            }
            None => Err(error::UnsafeInvokeZomeWorkspaceError::GuardDropped),
        }
    }
}

impl Drop for UnsafeInvokeZomeWorkspaceGuard<'_> {
    fn drop(&mut self) {
        if let Err(arc) = Arc::try_unwrap(self.workspace.take().expect("BUG: This has to be here"))
        {
            warn!(
                "Trying to drop UnsafeInvokeZomeWorkspace but there must be outstanding references"
            );
            // Wait on the lock to check if others have it
            tokio_safe_block_on::tokio_safe_block_on(
                arc.write(),
                std::time::Duration::from_secs(10),
            )
            .ok();
            // TODO: B-01648: Try to consume now hoping noone has taken a lock in the meantime
            Arc::try_unwrap(arc).expect(
                "UnsafeInvokeZomeWorkspace still has live references when workflow is finished",
            );
        }
    }
}

pub mod error {
    use thiserror::Error;
    #[derive(Error, Debug)]
    pub enum UnsafeInvokeZomeWorkspaceError {
        #[error(
            "The guard for this workspace has been dropped and this workspace is no loanger valid"
        )]
        GuardDropped,
    }
}

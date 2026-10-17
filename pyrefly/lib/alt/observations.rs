/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * This source code is licensed under the MIT license found in the LICENSE file.
 */

//! Query-scoped native observations. Types must be encoded with the live transaction;
//! these records are not runtime suppression or termination policies.
use pyrefly_config::error_kind::ErrorKind;
use pyrefly_types::types::Type;
use ruff_python_ast::name::Name;
use ruff_text_size::TextRange;
use crate::binding::binding::IsAsync;

#[derive(Debug, Clone)]
pub struct NativeDiagnostic {
    pub range: TextRange,
    pub kind: ErrorKind,
    pub message: String,
    pub details: Option<String>,
}

/// A diagnostic-free check is not proof of runtime applicability, especially for Any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeCallStatus { NoHardDiagnostics, HardDiagnostics }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextExitAwaitStatus { Synchronous, Awaitable, NotAwaitable }

#[derive(Debug, Clone)]
pub struct ContextExitCallObservation {
    pub result: Type,
    pub status: NativeCallStatus,
    pub awaitability: ContextExitAwaitStatus,
    pub diagnostics: Vec<NativeDiagnostic>,
}

#[derive(Debug, Clone)]
pub struct ContextExitObservation {
    pub range: TextRange,
    pub kind: IsAsync,
    pub receiver: Type,
    pub member_name: Name,
    pub member: Type,
    pub member_diagnostics: Vec<NativeDiagnostic>,
    pub normal: ContextExitCallObservation,
    pub exceptional: ContextExitCallObservation,
    /// Awaitability errors are emitted outside the argument collectors by the native solver.
    pub await_diagnostics: Vec<NativeDiagnostic>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalCallDecision {
    DeclaredOrInferredDivergence,
    NarrowedNeverCallee,
    InferredMethodReturn,
    NotImplementedBody,
    NonDivergentCallable,
    NoNativeCallableSignature,
}

#[derive(Debug, Clone)]
pub struct TerminalCallObservation {
    pub range: TextRange,
    pub callee: Type,
    /// The signature exposed by native `to_callable`; None does not prove non-callability.
    pub return_type: Option<Type>,
    /// None means the resolved callee has no top-level function metadata.
    pub return_is_inferred: Option<bool>,
    pub is_bound_method: bool,
    pub decision: TerminalCallDecision,
}

impl TerminalCallObservation {
    pub fn is_diverging(&self) -> bool {
        self.decision == TerminalCallDecision::DeclaredOrInferredDivergence
    }
}

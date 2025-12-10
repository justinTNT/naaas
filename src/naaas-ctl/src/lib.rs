//! NAAAS Control CLI - Command line interface for managing tenant unikernels
//! 
//! This library provides the core functionality for the NAAAS CLI,
//! including command parsing, HTTP client operations, and data models.

pub mod cli;
pub mod client;

pub use cli::{Cli, Commands};
pub use client::{NaaasClient, DeployRequest, Tenant};
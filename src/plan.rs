//! Treatment plan representation.
//!
//! This module provides the [`Plan`] struct for representing radiation therapy
//! treatment plans, including their associated dose-volume histograms.

use crate::Dvh;
use std::collections::HashMap;
use crate::traits::DvhCheck;

/// Represents a radiation therapy treatment plan.
///
/// A plan contains identification information, an optional name, and a collection
/// of dose-volume histograms (DVHs) associated with different structures.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Plan {
    /// Unique identifier for the treatment plan.
    pub id: String,
    /// Optional human-readable name for the treatment plan.
    pub name: Option<String>,
    /// Collection of DVHs mapped by structure name or identifier.
    pub dvhs: HashMap<String, Dvh>,
}

impl DvhCheck for Plan {
    fn dvh_check(&mut self) -> crate::Result<()> {
        for dvh in self.dvhs.values_mut() {
            dvh.dvh_check()?;
        }
        Ok(())
    }
}
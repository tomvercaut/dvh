//! Treatment plan representation.
//!
//! This module provides the [`Plan`] struct for representing radiation therapy
//! treatment plans, including their associated dose-volume histograms.

use crate::traits::DvhCheck;
use crate::{Dvh, MaxDose};
use std::collections::HashMap;

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

impl MaxDose for Plan {
    fn max_dose(&self) -> f64 {
        self.dvhs
            .values()
            .map(|dvh| dvh.max_dose())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
}

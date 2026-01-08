//! Patient information representation.
//!
//! This module provides the [`Patient`] struct for representing patient information
//! in radiation therapy contexts, including patient identification and associated
//! treatment plans.

use crate::name::Name;
use crate::plan::Plan;

/// Represents a patient in a radiation therapy context.
///
/// Contains patient identification, optional name information, and a collection
/// of associated treatment plans.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Patient {
    /// Unique identifier for the patient.
    pub patient_id: String,
    /// Optional structured name information for the patient.
    pub name: Option<Name>,
    /// Collection of treatment plans associated with this patient.
    pub plans: Vec<Plan>,
}

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

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;
    use crate::Dvh;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn test_patient_serialize_to_json() {
        let mut dvhs = HashMap::new();
        dvhs.insert(
            "PTV".to_string(),
            Dvh::default(),
        );

        let patient = Patient {
            patient_id: "P12345".to_string(),
            name: Some(Name {
                last: "Doe".to_string(),
                first: "John".to_string(),
                middle: "Michael".to_string(),
                prefix: "Dr.".to_string(),
                suffix: "Jr.".to_string(),
            }),
            plans: vec![Plan {
                id: "PLAN001".to_string(),
                name: Some("Treatment Plan 1".to_string()),
                dvhs,
            }],
        };

        let json = serde_json::to_string(&patient).expect("Failed to serialize patient");
        assert!(json.contains("P12345"));
        assert!(json.contains("Doe"));
        assert!(json.contains("John"));
        assert!(json.contains("PLAN001"));
    }

    #[test]
    fn test_patient_deserialize_from_json() {
        let json = r#"{
            "patient_id": "P67890",
            "name": {
                "last": "Smith",
                "first": "Jane",
                "middle": "Anne",
                "prefix": "Ms.",
                "suffix": "PhD"
            },
            "plans": [
                {
                    "id": "PLAN002",
                    "name": "Treatment Plan 2",
                    "dvhs": {}
                }
            ]
        }"#;

        let patient: Patient = serde_json::from_str(json).expect("Failed to deserialize patient");
        assert_eq!(patient.patient_id, "P67890");
        assert!(patient.name.is_some());
        let name = patient.name.unwrap();
        assert_eq!(name.last, "Smith");
        assert_eq!(name.first, "Jane");
        assert_eq!(name.middle, "Anne");
        assert_eq!(name.prefix, "Ms.");
        assert_eq!(name.suffix, "PhD");
        assert_eq!(patient.plans.len(), 1);
        assert_eq!(patient.plans[0].id, "PLAN002");
        assert_eq!(patient.plans[0].name, Some("Treatment Plan 2".to_string()));
    }

    #[test]
    fn test_patient_round_trip() {
        let original = Patient {
            patient_id: "P99999".to_string(),
            name: Some(Name {
                last: "Brown".to_string(),
                first: "Alice".to_string(),
                middle: "".to_string(),
                prefix: "".to_string(),
                suffix: "".to_string(),
            }),
            plans: vec![
                Plan {
                    id: "PLAN003".to_string(),
                    name: None,
                    dvhs: HashMap::new(),
                },
                Plan {
                    id: "PLAN004".to_string(),
                    name: Some("Secondary Plan".to_string()),
                    dvhs: HashMap::new(),
                },
            ],
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: Patient = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_patient_serialize_minimal() {
        let patient = Patient {
            patient_id: "P00001".to_string(),
            name: None,
            plans: vec![],
        };

        let json = serde_json::to_string(&patient).expect("Failed to serialize minimal patient");
        assert!(json.contains("P00001"));
        assert!(json.contains("null") || !json.contains("name"));
    }

    #[test]
    fn test_patient_deserialize_missing_optional_fields() {
        let json = r#"{
            "patient_id": "P11111",
            "plans": []
        }"#;

        let patient: Patient = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(patient.patient_id, "P11111");
        assert!(patient.name.is_none());
        assert_eq!(patient.plans.len(), 0);
    }
}


use dvh::Patient;
use std::fs;

#[test]
#[cfg(feature = "serde")]
fn test_integration_read_from_json() {
    let json_content = fs::read_to_string("tests/data/patient_data.json")
        .expect("Should have been able to read the file");

    let patient: Patient = serde_json::from_str(&json_content)
        .expect("Failed to deserialize patient from JSON");

    // Verify Patient info
    assert_eq!(patient.patient_id, "P-123");
    let name = patient.name.expect("Patient should have a name");
    assert_eq!(name.last, "Smith");
    assert_eq!(name.first, "John");

    // Verify Plans
    assert_eq!(patient.plans.len(), 2);

    // Check Plan 1
    let plan1 = &patient.plans[0];
    assert_eq!(plan1.id, "Plan-1");
    assert_eq!(plan1.name.as_deref(), Some("Initial Treatment"));
    assert!(plan1.dvhs.contains_key("PTV"));
    assert!(plan1.dvhs.contains_key("Rectum"));

    let ptv_dvh = &plan1.dvhs["PTV"];
    assert_eq!(ptv_dvh.len(), 6);
    assert_eq!(ptv_dvh.doses(), &[0.0, 10.0, 20.0, 30.0, 40.0, 50.0]);
    assert_eq!(ptv_dvh.volumes(), &[1.0, 1.0, 0.98, 0.95, 0.5, 0.0]);

    let rectum_dvh = &plan1.dvhs["Rectum"];
    assert_eq!(rectum_dvh.len(), 5);
    assert_eq!(rectum_dvh.doses(), &[0.0, 10.0, 20.0, 30.0, 40.0]);
    assert_eq!(rectum_dvh.volumes(), &[1.0, 0.5, 0.2, 0.1, 0.0]);

    // Check Plan 2
    let plan2 = &patient.plans[1];
    assert_eq!(plan2.id, "Plan-2");
    assert_eq!(plan2.name.as_deref(), Some("Boost"));
    assert!(plan2.dvhs.contains_key("PTV"));
    assert!(plan2.dvhs.contains_key("Bladder"));

    let ptv_dvh = &plan2.dvhs["PTV"];
    assert_eq!(ptv_dvh.len(), 5);
    assert_eq!(ptv_dvh.doses(), &[0.0, 5.0, 10.0, 15.0, 20.0]);
    assert_eq!(ptv_dvh.volumes(), &[1.0, 1.0, 1.0, 0.9, 0.0]);

    let bladder_dvh = &plan2.dvhs["Bladder"];
    assert_eq!(bladder_dvh.len(), 5);
    assert_eq!(bladder_dvh.doses(), &[0.0, 5.0, 10.0, 15.0, 20.0]);
    assert_eq!(bladder_dvh.volumes(), &[1.0, 0.8, 0.4, 0.1, 0.0]);
}

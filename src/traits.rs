/// Trait for validating dose-volume histogram data.
///
/// Implementors of this trait can perform validation and consistency checks
/// on DVH data, ensuring that dose and volume values are properly sorted,
/// non-negative, and meet other domain-specific requirements.
pub trait DvhCheck {
    /// Validates and checks the dose-volume histogram data.
    ///
    /// This method performs validation on DVH data structures, including checking
    /// that doses and volumes are properly sorted, non-negative, and satisfy
    /// other domain constraints. The method may modify the internal state to ensure
    /// data consistency (e.g., sorting unsorted data).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the DVH data is valid or has been successfully corrected.
    /// Returns an `Err` if validation fails or if the data cannot be corrected.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - DVH data contains invalid values (e.g., negative doses or volumes)
    /// - Volume values are not within the valid range (0.0 to 1.0) if the volume type is [Percent](dvh::VolumeType::Percent)
    /// - Data structures are inconsistent and cannot be automatically corrected

    fn dvh_check(&mut self) -> crate::Result<()>;
}

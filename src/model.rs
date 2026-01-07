use crate::Error;


/// Performs linear interpolation between two points.
///
/// # Parameters
/// - `x`: The x-coordinate at which to interpolate
/// - `x0`: The x-coordinate of the first point
/// - `x1`: The x-coordinate of the second point
/// - `y0`: The y-coordinate of the first point
/// - `y1`: The y-coordinate of the second point
///
/// # Returns
/// The interpolated y-value at x. If x0 equals x1, returns y0.
fn linear_interpolation(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    if x1 == x0 {
        return y0;
    }
    (x - x0) * (y1 - y0) / (x1 - x0) + y0
}

/// Represents the unit type for dose measurements.
///
/// # Variants
/// - `Gy`: Gray (default)
/// - `CGy`: Centigray
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DoseType {
    #[default]
    Gy,
    CGy,
}

/// Dose-Volume Histogram (DVH) structure for radiation therapy analysis.
///
/// A DVH represents the relationship between radiation dose and the volume
/// of a structure receiving that dose. The data is stored as parallel vectors
/// of dose and volume values.
///
/// # Fields
/// - `dose_type`: The unit type for dose measurements
/// - `d`: Vector of dose values
/// - `v`: Vector of volume values
/// - `is_sorted`: Whether the data is sorted by dose in ascending order
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dvh {
    // The unit type for dose
    pub dose_type: DoseType,
    // Doses
    d: Vec<f64>,
    // Volumes
    v: Vec<f64>,
    // Is the data sorted monotonically incrementally along the dose axis?
    is_sorted: bool,
}

impl Dvh {
    /// Creates a new empty DVH with the specified dose type.
    ///
    /// # Parameters
    /// - `dose_type`: The unit type for dose measurements
    ///
    /// # Returns
    /// A new empty DVH instance
    pub fn new(dose_type: DoseType) -> Dvh {
        Self {
            dose_type,
            d: Default::default(),
            v: Default::default(),
            is_sorted: true,
        }
    }

    /// Returns the number of dose-volume data points in the DVH.
    ///
    /// # Returns
    /// The number of data points
    pub fn len(&self) -> usize {
        self.d.len()
    }

    /// Checks if the DVH contains no data points.
    ///
    /// # Returns
    /// `true` if the DVH is empty, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.d.is_empty()
    }

    /// Adds a single dose-volume data point to the DVH.
    ///
    /// # Parameters
    /// - `d`: The dose value (must be non-negative)
    /// - `v`: The volume value (must be non-negative)
    ///
    /// # Returns
    /// `true` if the data point was added successfully, `false` if either value is negative
    pub fn add(&mut self, d: f64, v: f64) -> bool {
        if d < 0.0 {
            return false;
        }
        if v < 0.0 {
            return false;
        }
        self.is_sorted = false;
        self.d.push(d);
        self.v.push(v);
        true
    }

    /// Adds multiple dose-volume data points to the DVH from slices.
    ///
    /// # Parameters
    /// - `d`: Slice of dose values (all must be non-negative)
    /// - `v`: Slice of volume values (all must be non-negative)
    ///
    /// # Returns
    /// `true` if all data points were added successfully, `false` if the slices have different
    /// lengths or if any value is negative
    pub fn add_slice(&mut self, d: &[f64], v: &[f64]) -> bool {
        if d.len() != v.len() {
            return false;
        }
        for x in d {
            if *x < 0.0 {
                return false;
            }
        }
        for x in v {
            if *x < 0.0 {
                return false;
            }
        }

        self.is_sorted = false;
        self.d.extend_from_slice(d);
        self.v.extend_from_slice(v);
        true
    }

    /// Sorts the DVH data by dose values in ascending order.
    ///
    /// This method sorts both the dose and volume vectors together, maintaining
    /// the correspondence between dose-volume pairs. If the data is already sorted,
    /// this is a no-op.
    pub fn sort(&mut self) {
        if self.is_sorted {
            return;
        }
        let mut indices = (0..self.d.len()).collect::<Vec<_>>();
        indices.sort_unstable_by(|&i, &j| self.d[i].partial_cmp(&self.d[j]).unwrap());

        let d_sorted = indices.iter().map(|&i| self.d[i]).collect();
        let v_sorted = indices.iter().map(|&i| self.v[i]).collect();

        self.d = d_sorted;
        self.v = v_sorted;
        self.is_sorted = true;
    }

    /// Calculates the minimum dose received by a given volume (Dx query).
    ///
    /// This method performs linear interpolation to find the dose value at which
    /// the specified volume is covered. The DVH must be sorted before calling this method.
    ///
    /// # Parameters
    /// - `volume`: The volume for which to find the dose (must be non-negative)
    ///
    /// # Returns
    /// The dose value at the specified volume
    ///
    /// # Errors
    /// - `Error::NegativeVolume`: If the volume parameter is negative
    /// - `Error::DvhNoData`: If the DVH is empty
    /// - `Error::DvhInsufficientData`: If the DVH has fewer than 2 data points
    /// - `Error::DvhUnsorted`: If the DVH is not sorted
    /// - `Error::DvhDxLogic`: If an internal logic error occurs
    pub fn dx(&self, volume: f64) -> crate::Result<f64> {
        if volume < 0.0 {
            return Err(Error::NegativeVolume);
        }
        if self.is_empty() {
            return Err(Error::DvhNoData);
        }
        if self.len() < 2 {
            return Err(Error::DvhInsufficientData);
        }
        if !self.is_sorted {
            return Err(Error::DvhUnsorted);
        }

        let n = self.v.len();
        let mut x0 = self.v[n-1];
        let mut y0 = self.d[n-1];
        if volume <= x0 {
            return Ok(y0);
        }
        for (x1, y1) in self.v.iter().rev().zip(self.d.iter().rev()) {
            if volume >= x0 && volume <= *x1 {
                return Ok(linear_interpolation(volume, x0, *x1, y0, *y1));
            }
            x0 = *x1;
            y0 = *y1;
        }
        if volume > x0 {
            return Ok(y0);
        }

        Err(Error::DvhDxLogic)
    }

    /// Calculates the volume receiving at least the specified dose (Vx query).
    ///
    /// This method performs linear interpolation to find the volume value at the
    /// specified dose level. The DVH must be sorted before calling this method.
    ///
    /// # Parameters
    /// - `dose`: The dose level for which to find the volume (must be non-negative)
    ///
    /// # Returns
    /// The volume value at the specified dose
    ///
    /// # Errors
    /// - `Error::NegativeDose`: If the dose parameter is negative
    /// - `Error::DvhNoData`: If the DVH is empty
    /// - `Error::DvhInsufficientData`: If the DVH has fewer than 2 data points
    /// - `Error::DvhUnsorted`: If the DVH is not sorted
    /// - `Error::DvhVxLogic`: If an internal logic error occurs
    pub fn vx(&self, dose: f64) -> crate::Result<f64> {
        if dose < 0.0 {
            return Err(Error::NegativeDose);
        }
        if self.is_empty() {
            return Err(Error::DvhNoData);
        }
        if self.len() < 2 {
            return Err(Error::DvhInsufficientData);
        }
        if !self.is_sorted {
            return Err(Error::DvhUnsorted);
        }

        let n = self.d.len();
        let mut x0 = self.d[0];
        let mut y0 = self.v[0];
        if dose <= x0 {
            return Ok(y0);
        }
        for (x1, y1) in self.d.iter().zip(self.v.iter()) {
            if dose >= x0 && dose <= *x1 {
                return Ok(linear_interpolation(dose, x0, *x1, y0, *y1));
            }
            x0 = *x1;
            y0 = *y1;
        }
        if dose > self.d[n - 1] {
            return Ok(self.v[n - 1]);
        }
        Err(Error::DvhVxLogic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_interpolation_normal() {
        let result = linear_interpolation(5.0, 0.0, 10.0, 0.0, 100.0);
        assert_eq!(result, 50.0);
    }

    #[test]
    fn test_linear_interpolation_same_x() {
        let result = linear_interpolation(5.0, 10.0, 10.0, 20.0, 30.0);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_linear_interpolation_boundary() {
        let result = linear_interpolation(0.0, 0.0, 10.0, 0.0, 100.0);
        assert_eq!(result, 0.0);

        let result = linear_interpolation(10.0, 0.0, 10.0, 0.0, 100.0);
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_dvh_new() {
        let dvh = Dvh::new(DoseType::Gy);
        assert!(dvh.is_empty());
        assert_eq!(dvh.len(), 0);
        assert!(dvh.is_sorted);
    }

    #[test]
    fn test_dvh_new_cgy() {
        let dvh = Dvh::new(DoseType::CGy);
        assert!(dvh.is_empty());
        assert!(matches!(dvh.dose_type, DoseType::CGy));
    }

    #[test]
    fn test_dvh_len_and_is_empty() {
        let mut dvh = Dvh::new(DoseType::Gy);
        assert_eq!(dvh.len(), 0);
        assert!(dvh.is_empty());

        dvh.add(1.0, 100.0);
        assert_eq!(dvh.len(), 1);
        assert!(!dvh.is_empty());

        dvh.add(2.0, 90.0);
        assert_eq!(dvh.len(), 2);
        assert!(!dvh.is_empty());
    }

    #[test]
    fn test_dvh_add_valid() {
        let mut dvh = Dvh::new(DoseType::Gy);
        assert!(dvh.add(1.0, 100.0));
        assert_eq!(dvh.len(), 1);
        assert!(!dvh.is_sorted);
    }

    #[test]
    fn test_dvh_add_negative_dose() {
        let mut dvh = Dvh::new(DoseType::Gy);
        assert!(!dvh.add(-1.0, 100.0));
        assert_eq!(dvh.len(), 0);
    }

    #[test]
    fn test_dvh_add_negative_volume() {
        let mut dvh = Dvh::new(DoseType::Gy);
        assert!(!dvh.add(1.0, -100.0));
        assert_eq!(dvh.len(), 0);
    }

    #[test]
    fn test_dvh_add_zero_values() {
        let mut dvh = Dvh::new(DoseType::Gy);
        assert!(dvh.add(0.0, 0.0));
        assert_eq!(dvh.len(), 1);
    }

    #[test]
    fn test_dvh_add_slice_valid() {
        let mut dvh = Dvh::new(DoseType::Gy);
        let doses = vec![1.0, 2.0, 3.0];
        let volumes = vec![100.0, 90.0, 80.0];
        assert!(dvh.add_slice(&doses, &volumes));
        assert_eq!(dvh.len(), 3);
        assert!(!dvh.is_sorted);
    }

    #[test]
    fn test_dvh_add_slice_mismatched_length() {
        let mut dvh = Dvh::new(DoseType::Gy);
        let doses = vec![1.0, 2.0];
        let volumes = vec![100.0, 90.0, 80.0];
        assert!(!dvh.add_slice(&doses, &volumes));
        assert_eq!(dvh.len(), 0);
    }

    #[test]
    fn test_dvh_add_slice_negative_dose() {
        let mut dvh = Dvh::new(DoseType::Gy);
        let doses = vec![1.0, -2.0, 3.0];
        let volumes = vec![100.0, 90.0, 80.0];
        assert!(!dvh.add_slice(&doses, &volumes));
        assert_eq!(dvh.len(), 0);
    }

    #[test]
    fn test_dvh_add_slice_negative_volume() {
        let mut dvh = Dvh::new(DoseType::Gy);
        let doses = vec![1.0, 2.0, 3.0];
        let volumes = vec![100.0, -90.0, 80.0];
        assert!(!dvh.add_slice(&doses, &volumes));
        assert_eq!(dvh.len(), 0);
    }

    #[test]
    fn test_dvh_add_slice_empty() {
        let mut dvh = Dvh::new(DoseType::Gy);
        let doses: Vec<f64> = vec![];
        let volumes: Vec<f64> = vec![];
        assert!(dvh.add_slice(&doses, &volumes));
        assert_eq!(dvh.len(), 0);
    }

    #[test]
    fn test_dvh_sort() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(3.0, 80.0);
        dvh.add(1.0, 100.0);
        dvh.add(2.0, 90.0);

        dvh.sort();

        assert!(dvh.is_sorted);
        assert_eq!(dvh.d, vec![1.0, 2.0, 3.0]);
        assert_eq!(dvh.v, vec![100.0, 90.0, 80.0]);
    }

    #[test]
    fn test_dvh_sort_already_sorted() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(1.0, 100.0);
        dvh.add(2.0, 90.0);
        dvh.sort();

        // Sort again should not change anything
        dvh.sort();

        assert!(dvh.is_sorted);
        assert_eq!(dvh.d, vec![1.0, 2.0]);
        assert_eq!(dvh.v, vec![100.0, 90.0]);
    }

    #[test]
    fn test_dvh_dx_negative_volume() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(1.0, 100.0);
        dvh.add(2.0, 90.0);
        dvh.sort();

        let result = dvh.dx(-10.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::NegativeVolume));
    }

    #[test]
    fn test_dvh_dx_empty() {
        let dvh = Dvh::new(DoseType::Gy);
        let result = dvh.dx(50.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::DvhNoData));
    }

    #[test]
    fn test_dvh_dx_insufficient_data() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(1.0, 100.0);
        dvh.sort();

        let result = dvh.dx(50.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::DvhInsufficientData));
    }

    #[test]
    fn test_dvh_dx_unsorted() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(1.0, 100.0);
        dvh.add(2.0, 90.0);
        // Don't sort

        let result = dvh.dx(95.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::DvhUnsorted));
    }

    #[test]
    fn test_dvh_dx_interpolation() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(0.0, 100.0);
        dvh.add(10.0, 80.0);
        dvh.sort();

        let result = dvh.dx(90.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_dvh_dx_below_minimum() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(0.0, 100.0);
        dvh.add(10.0, 80.0);
        dvh.sort();

        let result = dvh.dx(70.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10.0);
    }

    #[test]
    fn test_dvh_dx_above_maximum() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(0.0, 100.0);
        dvh.add(10.0, 80.0);
        dvh.sort();

        let result = dvh.dx(110.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_dvh_dx_exact_match() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(0.0, 100.0);
        dvh.add(5.0, 90.0);
        dvh.add(10.0, 80.0);
        dvh.sort();

        let result = dvh.dx(90.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_dvh_dx_multiple_points() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(0.0, 100.0);
        dvh.add(5.0, 90.0);
        dvh.add(10.0, 80.0);
        dvh.add(15.0, 70.0);
        dvh.sort();

        // Test interpolation between different segments
        let result = dvh.dx(85.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 7.5);

        let result = dvh.dx(79.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10.5);

        let result = dvh.dx(71.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 14.5);
    }

    #[test]
    fn test_dvh_vx_negative_dose() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(1.0, 100.0);
        dvh.add(2.0, 90.0);
        dvh.sort();

        let result = dvh.vx(-1.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::NegativeDose));
    }

    #[test]
    fn test_dvh_vx_empty() {
        let dvh = Dvh::new(DoseType::Gy);
        let result = dvh.vx(5.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::DvhNoData));
    }

    #[test]
    fn test_dvh_vx_insufficient_data() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(1.0, 100.0);
        dvh.sort();

        let result = dvh.vx(1.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::DvhInsufficientData));
    }

    #[test]
    fn test_dvh_vx_unsorted() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(1.0, 100.0);
        dvh.add(2.0, 90.0);
        // Don't sort

        let result = dvh.vx(1.5);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::DvhUnsorted));
    }

    #[test]
    fn test_dvh_vx_below_minimum() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(5.0, 100.0);
        dvh.add(10.0, 80.0);
        dvh.sort();

        let result = dvh.vx(3.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100.0);
    }

    #[test]
    fn test_dvh_vx_above_maximum() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(5.0, 100.0);
        dvh.add(10.0, 80.0);
        dvh.sort();

        let result = dvh.vx(15.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 80.0);
    }

    #[test]
    fn test_dvh_vx_exact_match() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(0.0, 100.0);
        dvh.add(5.0, 90.0);
        dvh.add(10.0, 80.0);
        dvh.sort();

        let result = dvh.vx(5.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 90.0);
    }

    #[test]
    fn test_dvh_vx_interpolation() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(0.0, 100.0);
        dvh.add(10.0, 80.0);
        dvh.sort();

        let result = dvh.vx(5.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 90.0);

        let result = dvh.vx(2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 96.0);

        let result = dvh.vx(8.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 84.0);
    }

    #[test]
    fn test_dvh_vx_multiple_points() {
        let mut dvh = Dvh::new(DoseType::Gy);
        dvh.add(0.0, 100.0);
        dvh.add(5.0, 90.0);
        dvh.add(10.0, 80.0);
        dvh.add(15.0, 70.0);
        dvh.sort();

        // Test interpolation between different segments
        let result = dvh.vx(7.5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 85.0);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_dvh_serde() {
        let mut dvh = Dvh::new(DoseType::CGy);
        dvh.add(0.0, 100.0);
        dvh.add(10.0, 80.0);
        dvh.sort();

        let serialized = serde_json::to_string(&dvh).unwrap();
        let deserialized: Dvh = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.dose_type, DoseType::CGy);
        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized.dx(90.0).unwrap(), 5.0);
    }
}


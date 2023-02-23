/* ---------------------------- External imports ---------------------------- */
use physical_constants as CST;
use anyhow::Result;

/* ---------------------------- Internal imports ---------------------------- */


/* -------------------------------------------------------------------------- */
/*                            FUNCTION DEFINITIONS                            */
/* -------------------------------------------------------------------------- */
// Ge units => radical / 100eV / incident particle
// Kr units => mol/l/Gy
pub fn ge_to_kr(ge:f64) -> Result<f64> {
    let d:f64 = 1.0; // Solvent density [kg/l]
    Ok(ge * d / CST::ELEMENTARY_CHARGE / 100.0 / CST::AVOGADRO_CONSTANT)
}

/* -------------------------------------------------------------------------- */
/*                                   TESTING                                  */
/* -------------------------------------------------------------------------- */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ge_to_kr() {
        // Test against MatLab computed values from R. Labarbe
        assert_float_relative_eq!(ge_to_kr(2.80).unwrap(), 2.9020e-07, 1e-5);
        assert_float_relative_eq!(ge_to_kr(0.62).unwrap(), 6.4258e-08, 1e-5);
        assert_float_relative_eq!(ge_to_kr(0.47).unwrap(), 4.8712e-08, 1e-5);
        assert_float_relative_eq!(ge_to_kr(0.73).unwrap(), 7.5659e-08, 1e-5);
    }
}
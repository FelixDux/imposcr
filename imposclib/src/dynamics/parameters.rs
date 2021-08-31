use super::model_types::Frequency as Frequency;
use super::model_types::Phase as Phase;
use super::model_types::Distance as Distance;
use super::model_types::Coefficient as Coefficient;
use super::model_types::ParameterError as ParameterError;

#[derive(Debug, Copy, Clone)]
pub struct Parameters {
	forcing_frequency: Frequency,
	coefficient_of_restitution: Coefficient,
	obstacle_offset: Distance,
	gamma: Coefficient,
	maximum_periods: u32 // maximum forcing periods to detect impact
}

impl Parameters {
    fn new(frequency: Frequency, offset: Distance, r: Coefficient, max_periods: u32) -> Result<Parameters, Vec<ParameterError>> {
        let mut error_list: Vec<ParameterError> = vec![];

        if frequency == 0.0 {
            error_list.push(ParameterError::ZeroForcingFrequency);
        }

        if 0.0 > frequency {
            error_list.push(ParameterError::NegativeForcingFrequency{frequency: frequency});
        }

        if frequency == 1.0 {
            error_list.push(ParameterError::ResonantForcingFrequency{frequency: frequency});
        }

        if 1.0 < r {
            error_list.push(ParameterError::LargeCoefficientOfRestitution{coefficient: r});
        }

        if 0.0 > r {
            error_list.push(ParameterError::NegativeCoefficientOfRestitution{coefficient: r});
        }

        if max_periods == 0 {
            error_list.push(ParameterError::ZeroMaximumPeriods);
        }

        if error_list.len() > 0 {
            return Err(error_list);
        }

        Ok(Parameters{forcing_frequency: frequency, obstacle_offset: offset, coefficient_of_restitution: r, maximum_periods: max_periods, gamma: 1.0/(1.0 - frequency.powf(2.0))})
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug)]
    struct ParameterErrorTest {
        forcing_frequency: Frequency,
        coefficient_of_restitution: Coefficient,
        obstacle_offset: Distance,
        maximum_periods: u32,
        expected_errors: usize
    }
    
    fn test_parameter_errors() {

        let ParameterErrorTests = vec![
            ParameterErrorTest{forcing_frequency: 2.8, coefficient_of_restitution: 0.0, obstacle_offset: 0.1, maximum_periods: 100, expected_errors: 0},
            ParameterErrorTest{forcing_frequency: 2.8, coefficient_of_restitution: 0.0, obstacle_offset: 0.1, maximum_periods: 0, expected_errors: 1},
            ParameterErrorTest{forcing_frequency: -2.8, coefficient_of_restitution: 0.8, obstacle_offset: 0.1, maximum_periods: 100, expected_errors: 1},
            ParameterErrorTest{forcing_frequency: 2.8, coefficient_of_restitution: 0.8, obstacle_offset: -0.1, maximum_periods: 100, expected_errors: 0},
            ParameterErrorTest{forcing_frequency: 2.8, coefficient_of_restitution: -0.5, obstacle_offset: 0.1, maximum_periods: 100, expected_errors: 1},
            ParameterErrorTest{forcing_frequency: 0.0, coefficient_of_restitution: 2.3, obstacle_offset: 0.1, maximum_periods: 100, expected_errors: 2},
            ParameterErrorTest{forcing_frequency: 1.0, coefficient_of_restitution: 1.2, obstacle_offset: -0.1, maximum_periods: 0, expected_errors: 3},
        ];

        for data in ParameterErrorTests.iter() {
            let result = Parameters::new(data.forcing_frequency, data.obstacle_offset, data.coefficient_of_restitution, data.maximum_periods);

            if data.expected_errors == 0 {
                assert!(result.is_ok());
            }
            else {
                let errors = result.unwrap_err();

                assert_eq!(errors.len(), data.expected_errors);
            }
        }
    }
}
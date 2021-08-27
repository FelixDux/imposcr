#![crate_name = "doc"]
#![feature(result_contains_err)]

use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub enum PhaseError {
    ZeroForcingFrequency,
    NegativeForcingFrequency {frequency: f64 }
} 

/// For a given forcing period, converts between simulation time, normalised forcing phase and
/// number of periods into a simulation.
#[derive(Debug)]
pub struct PhaseConverter {
    period: f64
}

fn new_phase_converter(frequency: f64) -> Result<PhaseConverter, PhaseError> {
    if frequency == 0.0 {
        Err(PhaseError::ZeroForcingFrequency)
    } else if frequency < 0.0 {
        Err(PhaseError::NegativeForcingFrequency{frequency: frequency})
    } else {
        Ok(PhaseConverter{ period: PI * 2.0f64/ frequency})
    }
}

impl PhaseConverter {

// func NewPhaseConverter(frequency float64) (*PhaseConverter, error) {
//     if (frequency == 0) {
//         return nil, parameters.ZeroForcingFrequencyError(frequency)
//     }

//     if (frequency < 0) {
//         return nil, parameters.NegativeForcingFrequencyError(frequency)
//     }

//     period := 2.0 * math.Pi / frequency

//     return &PhaseConverter{
//         Period: period,
//     }, nil
// }

// func (converter PhaseConverter) TimeToPhase(simtime float64) float64 {
//     scaled_time := simtime / converter.Period

//     return scaled_time - math.Floor(scaled_time)
// }

// func (converter PhaseConverter) TimeIntoCycle (phase float64) float64 {
//     return phase * converter.Period
// }

// func (converter PhaseConverter) ForwardToPhase (starttime float64, phase float64) float64 {
//     phase_change := phase - converter.TimeToPhase(starttime)

//     if (phase_change < 0) {
//         phase_change++
//     }

//     return starttime + converter.Period * phase_change
// }

// func (converter PhaseConverter) DifferenceInPeriods (starttime float64, endtime float64) int {
//     return int(math.Round(math.Abs(endtime - starttime)/converter.Period))
// }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_frequency_returns_error() {
        assert_eq!(new_phase_converter(0.0).unwrap_err(), PhaseError::ZeroForcingFrequency);
    }

    #[test]
    fn negative_frequency_returns_error() {
        let frequency = -1.0;
        assert_eq!(new_phase_converter(frequency).unwrap_err(), PhaseError::NegativeForcingFrequency{frequency});
    }

}
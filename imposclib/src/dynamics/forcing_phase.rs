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

    fn time_to_phase(self, simtime: f64) -> f64 {
        let scaled_time = simtime / self.period;

        scaled_time - scaled_time.floor()
    }

fn time_into_cycle (self, phase: f64) -> f64 {
    phase * self.period
}

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

    #[test]
    fn time_converts_to_phase_correctly() -> Result<(), PhaseError> {
        let converter = new_phase_converter(PI)?;

        let time = 3.0;
        let expected = 0.5;

        assert_eq!(converter.time_to_phase(time), expected);

        Ok(())
    }

    #[test]    
    fn convert_time_into_cycle() -> Result<(), PhaseError> {
        let phase = 0.80;
        let period = 1.25;
        let expected = 1.0;
        let f = 2.0*PI/period;
    
        assert_eq!(new_phase_converter(f)?.time_into_cycle(phase), expected);
    
        Ok(())
    }
    
    // var ints = [] int {1, 2, 4, 5, 16}
    // var frequencies = [] float64 {4.89, 2.76}
    // var start_time = 0.02
    
    // const tol = 1e-6
    // var opt = cmp.Comparer(func(x, y float64) bool {
    //     return math.Abs(x-y) < tol
    // })
    
    // func TestShiftTimeInPeriods(t *testing.T) {
    //     for _, f := range frequencies {
    //         conv, _ := NewPhaseConverter(f)
    
    //         for _, i := range ints {
    //             time_shift := float64(i)*conv.Period
    
    //             shifted_time := time_shift + start_time
    
    //             new_time := conv.TimeIntoCycle(conv.TimeToPhase(shifted_time))
    
    //             if !cmp.Equal(new_time, start_time, opt) {
    //                 t.Errorf("Converter with frequency %g does not convert consistently in both directions (start time %g, time shift %g, end time %g, %d periods)",
    //                         f, start_time, time_shift, new_time, i)
    //             }
    
    //             n := conv.DifferenceInPeriods(start_time, shifted_time)
    //             if i != n {
    //                 t.Errorf("Converter with frequency %g does not return correct number of periods %g between times %g and %g: expected %d, got %d",
    //                         f, conv.Period, start_time, shifted_time, i, n)                
    //             }
    //         }
    //     }
    // }
    
    // func TestForwardToPhase(t *testing.T) {
    //     for _, f := range frequencies {
    //         conv, _ := NewPhaseConverter(f)
    
    //         for _, i := range ints {
    //             phase := 0.6
    //             small_time := 0.2
    //             big_time := 0.8
    
    //             time_delta := float64(i) * conv.Period
    
    //             new_small := conv.ForwardToPhase(time_delta + small_time, phase)
    //             new_small_phase := conv.TimeToPhase(new_small)
    
    //             if !cmp.Equal(phase, new_small_phase, opt) {
    //                 t.Errorf("Phase converter with frequency %g runs forward time %g from time %g to phase %g, expected %g", f, time_delta, small_time, new_small_phase, phase)
    //             }
    
    //             new_big := conv.ForwardToPhase(time_delta + big_time, phase)
    //             new_big_phase := conv.TimeToPhase(new_big)
    
    //             if !cmp.Equal(phase, new_big_phase, opt) {
    //                 t.Errorf("Phase converter with frequency %g runs forward time %g from time %g to phase %g, expected %g", f, time_delta, big_time, new_big_phase, phase)
    //             }
    //         }
    //     }
    // }
    
}
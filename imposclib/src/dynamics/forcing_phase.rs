#![crate_name = "doc"]
#![feature(result_contains_err)]

use std::f64::consts::PI;
use float_eq::assert_float_eq;

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

    fn time_to_phase(&self, simtime: f64) -> f64 {
        let scaled_time = simtime / self.period;

        scaled_time - scaled_time.floor()
    }

    fn time_into_cycle(&self, phase: f64) -> f64 {
        phase * self.period
    }

    fn forward_to_phase(&self, starttime: f64, phase: f64) -> f64 {
        let mut phase_change = phase - self.time_to_phase(starttime);

        if phase_change < 0.0 {
            phase_change += 1.0;
        }

        starttime + self.time_into_cycle(phase_change)
    }

    fn difference_in_periods (&self, starttime: f64, endtime: f64) -> i32 {
        ((endtime - starttime).abs()/self.period).floor() as i32
    }
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
        
    #[test]
    fn test_shift_time_in_periods() -> Result<(), PhaseError> {
    
        let ints = vec![1, 2, 4, 5, 16];
        let frequencies: Vec<f64> = vec![4.89, 2.76];
        let start_time = 0.02;
        
        const TOL: f64 = 1e-6;

        let mut result = Ok(());

        let inner_test = |i: i32, frequency: f64| -> Result<(), PhaseError> {
            let conv = new_phase_converter(frequency)?;
            let time_shift = (i as f64)*conv.period;

            let shifted_time = time_shift + start_time;

            let new_time = conv.time_into_cycle(conv.time_to_phase(shifted_time));

            assert_float_eq!(new_time, start_time, abs <= TOL);

            let n = conv.difference_in_periods(start_time, shifted_time);
            assert_eq!(i, n);

            Ok(())
        };

        for f in &frequencies {    
            for i in &ints {
                result = inner_test(*i, *f);
            }
        }
        
        result
    }
    
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
#![crate_name = "imposclib"]
#![feature(result_contains_err)]

use std::f64::consts::PI;

pub type Time = f64;
pub type Phase = f64;
pub type Frequency = f64;

#[derive(Debug, PartialEq)]
/// Error modes for initialising a `PhaseConverter`.
/// 
/// A `PhaseConverter` must be initialised by a strictly positive forcing frequency
pub enum PhaseError {
    ZeroForcingFrequency,
    NegativeForcingFrequency {frequency: Frequency }
} 

/// For a given forcing period, converts between simulation time, normalised forcing phase and
/// number of periods into a simulation.
#[derive(Debug)]
pub struct PhaseConverter {
    /// The forcing period for the system, which must be strictly positive.
    period: Time
}

impl PhaseConverter {
    /// Returns a phase converter for a specified forcing frequency
    /// 
    /// # Arguments
    /// 
    /// * `frequency` - A strictly positive forcing frequency
    /// 
    /// # Examples
    /// 
    /// ```
    /// let converter = PhaseConverter(3.87);
    /// ```
    fn new(frequency: Frequency) -> Result<PhaseConverter, PhaseError> {
        if frequency == 0.0 {
            Err(PhaseError::ZeroForcingFrequency)
        } else if frequency < 0.0 {
            Err(PhaseError::NegativeForcingFrequency{frequency: frequency})
        } else {
            Ok(PhaseConverter{ period: PI * 2.0f64/ frequency})
        }
    }

    fn time_to_phase(&self, simtime: Time) -> Phase {
        let scaled_time = simtime / self.period;

        scaled_time - scaled_time.floor()
    }

    fn time_into_cycle(&self, phase: Phase) -> Time {
        phase * self.period
    }

    fn forward_to_phase(&self, starttime: Time, phase: Phase) -> Time {
        let mut phase_change = phase - self.time_to_phase(starttime);

        if phase_change < 0.0 {
            phase_change += 1.0;
        }

        starttime + self.time_into_cycle(phase_change)
    }

    fn difference_in_periods (&self, starttime: Time, endtime: Time) -> i32 {
        ((endtime - starttime).abs()/self.period).floor() as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    #[test]
    fn zero_frequency_returns_error() {
        assert_eq!(PhaseConverter::new(0.0).unwrap_err(), PhaseError::ZeroForcingFrequency);
    }

    #[test]
    fn negative_frequency_returns_error() {
        let frequency = -1.0;
        assert_eq!(PhaseConverter::new(frequency).unwrap_err(), PhaseError::NegativeForcingFrequency{frequency});
    }

    #[test]
    fn time_converts_to_phase_correctly() -> Result<(), PhaseError> {
        let converter = PhaseConverter::new(PI)?;

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
    
        assert_eq!(PhaseConverter::new(f)?.time_into_cycle(phase), expected);
    
        Ok(())
    }

    fn wrap_test<F>(inner_test: F) -> Result<(), PhaseError>
    where F: Fn(i32, f64) -> Result<(), PhaseError> {
    
        let ints = vec![1, 2, 4, 5, 16];
        let frequencies: Vec<f64> = vec![4.89, 2.76];

        let mut result = Ok(());

        for f in &frequencies {    
            for i in &ints {
                result = inner_test(*i, *f);
            }
        }
        
        result
    }
        
    #[test]
    fn test_shift_time_in_periods() -> Result<(), PhaseError> {

        let inner_test = |i: i32, frequency: f64| -> Result<(), PhaseError> {
            let start_time = 0.02;
            
            const TOL: f64 = 1e-6;

            let conv = PhaseConverter::new(frequency)?;
            let time_shift = (i as f64)*conv.period;

            let shifted_time = time_shift + start_time;

            let new_time = conv.time_into_cycle(conv.time_to_phase(shifted_time));

            assert_float_eq!(new_time, start_time, abs <= TOL);

            let n = conv.difference_in_periods(start_time, shifted_time);
            assert_eq!(i, n);

            Ok(())
        };

        wrap_test(inner_test)
    }
    
    #[test]
    fn test_forward_to_phase() -> Result<(), PhaseError> {
        let inner_test = |i: i32, frequency: f64| -> Result<(), PhaseError> {
            let phase = 0.6;
            let small_time = 0.2;
            let big_time = 0.8;
            
            const TOL: f64 = 1e-6;

            let conv = PhaseConverter::new(frequency)?;

            let time_delta = i as f64 * conv.period;

            let new_small = conv.forward_to_phase(time_delta + small_time, phase);
            let new_small_phase = conv.time_to_phase(new_small);

            assert_float_eq!(phase, new_small_phase, abs <= TOL);

            let new_big = conv.forward_to_phase(time_delta + big_time, phase);
            let new_big_phase = conv.time_to_phase(new_big);

            assert_float_eq!(phase, new_big_phase, abs <= TOL);

            Ok(())
        };

        wrap_test(inner_test)
    }    
}
use std::fmt;

pub type Time = f64;
pub type Phase = f64;
pub type Frequency = f64;
pub type Velocity = f64;
pub type Distance = f64;
pub type Coefficient = f64;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ParameterError {
    ZeroForcingFrequency,
    NegativeForcingFrequency {frequency: Frequency },
    ResonantForcingFrequency {frequency: Frequency },
    LargeCoefficientOfRestitution {coefficient: Coefficient},
    NegativeCoefficientOfRestitution {coefficient: Coefficient},
    ZeroMaximumPeriods {periods: u32}
}

// Displaying error modes
impl fmt::Display for ParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParameterError::ZeroForcingFrequency => write!(f, "Forcing frequency cannot be zero"),
            ParameterError::NegativeForcingFrequency{ref frequency} => write!(f, "The model cannot handle negative forcing frequencies {:?}", frequency),
            ParameterError::ResonantForcingFrequency{ref frequency} => write!(f, "A forcing frequency of {:?} is a resonant case with unbounded solutions", frequency),
            ParameterError::LargeCoefficientOfRestitution{ref coefficient} => write!(f, "A coefficient of restitution of {:?} > 1 will generate unbounded solutions", coefficient),
            ParameterError::NegativeCoefficientOfRestitution{ref coefficient} => write!(f, "A coefficient of restitution of {:?} < 0> will generate unphysical solutions", coefficient),
            ParameterError::ZeroMaximumPeriods{ref periods} => write!(f, "Maximum number of forcing periods {:?} to detect impact must be > 0", periods)
        }
    }
}
use super::forcing_phase::PhaseConverter as PhaseConverter;
use super::model_types::Time as Time;
use super::model_types::Phase as Phase;
use super::model_types::Velocity as Velocity;
use float_eq::FloatEq;

/// Each impact is uniquely specified by two parameters:
/// The `phase` (`time` modulo and scaled by the forcing period) at
/// which the impact occurs
/// The `velocity` of the impact, which cannot be negative
///
/// In addition, we also record the actual `time` of the impact.
///
/// Because the `phase` is periodic and the `velocity` non-negative, the surface on which
/// impacts are defined is a half-cylinder. Whether a zero-velocity impact
/// is physically meaningful depends on the value of the `phase` and on 
/// the offset of the obstacle from the centre of motion.
///
#[derive(Copy, Clone)]
pub struct SimpleImpact {
	phase: Phase,
	velocity: Velocity
}

#[derive(Copy, Clone)]
pub struct Impact {
	simple_impact: SimpleImpact,
	time: Time
}

#[derive(Debug)]
pub struct ImpactGenerator {
	converter: PhaseConverter
}

impl ImpactGenerator
{
	pub fn new(converter: PhaseConverter) -> ImpactGenerator {
		ImpactGenerator{converter: converter}
	}

	pub fn generate(&self, impact_time: Time, impact_velocity: Velocity) -> Impact  {
			Impact{time: impact_time, simple_impact: SimpleImpact {
				phase: self.converter.time_to_phase(impact_time), 
				velocity: impact_velocity}
			}
	}
}

type SimpleImpactComparer = dyn Fn(SimpleImpact, SimpleImpact) -> bool;

// Compare phase using absolute tolerance, velocity using relative tolerance
fn simple_impact_comparer(tolerance: SimpleImpact) -> Box<SimpleImpactComparer> {
	let inverse_phase_tolerance = 1.0 - tolerance.phase;
	Box::new(move |x: SimpleImpact, y: SimpleImpact| -> bool {
		// Compare phases
		if !x.phase.eq_abs(&y.phase, &tolerance.phase) {
			// Account for periodicity (i.e. 0 and 1 are the same)
			if x.phase.eq_abs(&y.phase, &inverse_phase_tolerance) {
				return false;
			}
		}
		
		x.velocity.eq_rmax(&y.velocity, &tolerance.velocity)
	})
}

pub type ImpactComparer = dyn Fn(Impact, Impact) -> bool;

pub fn impact_comparer(tolerance: SimpleImpact) -> Box<ImpactComparer> {
	let comparer = simple_impact_comparer(tolerance);

	Box::new(move |x: Impact, y: Impact| -> bool {
		comparer(x.simple_impact, y.simple_impact)
	})
}

fn default_impact_comparer() -> Box<ImpactComparer> 
{
	let tol = 1e-3;
	impact_comparer(SimpleImpact{phase: tol, velocity: tol})
}

impl Impact{
	pub fn phase(&self) -> Phase {
		self.simple_impact.phase
	}

	pub fn velocity(&self) -> Velocity {
		self.simple_impact.velocity
	}

	pub fn time(&self) -> Time {
		self.time
	}

	pub fn dual_impact(&self, coefficient_of_restitution: f64) -> Impact {
		// Returns the dual of an impact. If an impact is the image of a zero-velocity impact, then
		// its dual is the pre-image of the same zero-velocity impact. This is only well-defined for
		// non-zero coefficient of restitution. For the zero-restitution case, all impacts behave like
		// zero-velocity impacts anyway.

		if coefficient_of_restitution > 0.0 {
			Impact{
				simple_impact: SimpleImpact{phase: 1.0 - self.simple_impact.phase, 
				velocity: self.simple_impact.velocity / coefficient_of_restitution}, 
				time: -self.time}
		} else {
			Impact{
				simple_impact: SimpleImpact{
					phase: 1.0 - self.simple_impact.phase, 
					velocity: 0.0}, 
					time: -self.time}
		}
	}
}

type ImpactTransform = dyn Fn(Impact) -> Impact;

fn dual_maker(coefficient_of_restitution: f64) -> Box<ImpactTransform> {
	Box::new(move |i: Impact| -> Impact {
		i.dual_impact(coefficient_of_restitution)
	})
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn default_impact_comparison() -> () {
		let converter = PhaseConverter::new(2.0).unwrap();

		let generator = ImpactGenerator{converter: converter};

		let comparer = default_impact_comparer();

		let check_equal = |x: Impact, y: Impact, expected: bool| -> () {
			assert_eq!(comparer(x, y), expected)
		};

		let impact1 = generator.generate(0.0, 1.0);
		let impact2 = generator.generate(0.0001, 1.0001);
		let impact3 = generator.generate(0.3, 0.2);
		let impact4 = generator.generate(1.0000001*converter.get_period(), 1.0);
		let impact5 = generator.generate(0.99999*converter.get_period(), 1.0);
	
		check_equal(impact1, impact2, true);
		check_equal(impact3, impact2, false);
		check_equal(impact1, impact4, true);
		check_equal(impact1, impact5, true);
	}

	#[test]
	fn test_impact_dual() {
		let converter = PhaseConverter::new(2.0).unwrap();

		let generator = ImpactGenerator{converter: converter};

		let impact = generator.generate(0.3, 1.2);

		struct Test {
			r: f64,
			expected_v: f64
		}

		impl Test {
			fn run(&self, impact: Impact) -> () {
				let dual = impact.dual_impact(self.r);

				assert_eq!(dual.phase(), 1.0 - impact.phase());
				assert_eq!(dual.velocity(), self.expected_v);
			}
		}

		let tests = vec![
		Test{r: 0.8, expected_v: impact.velocity()/0.8},
		Test{r: 0.0, expected_v: 0.0}];

		for test in tests.iter() {
			test.run(impact);
		}
	}
}
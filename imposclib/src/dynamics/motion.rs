//
// Time evolution of the system from one impact to the next
//
use super::parameters::Parameters as Parameters;
use super::forcing_phase::PhaseConverter as PhaseConverter;
use super::model_types::Time as Time;
use super::model_types::Distance as Distance;
use super::model_types::Velocity as Velocity;
use super::model_types::Coefficient as Coefficient;
use super::impact::Impact as Impact;
use super::sticking::Sticking as Sticking;
use super::impact::ImpactGenerator as ImpactGenerator;

#[derive(Debug, Copy, Clone)]
pub struct StateOfMotion {
	// 	State and phase variables for the motion between impacts
	time: Time,
	displacement: Distance,
	velocity: Velocity
}

impl StateOfMotion {
    pub fn time(&self) -> Time {
        self.time
    }

    pub fn displacement(&self) -> Distance {
        self.displacement
    }

    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn constrain(&self, offset: Distance) -> StateOfMotion {
        StateOfMotion{displacement: if offset < self.displacement {offset} else{self.displacement}, ..*self}
    }
}

#[derive(Debug)]
pub struct LongExcursionChecker {
    period: Time,
    from_time: Time,
    maximum_periods: u32
}

impl LongExcursionChecker {
    fn new(maximum_periods: u32, converter: PhaseConverter, from_time: Time) -> LongExcursionChecker {
        LongExcursionChecker{period: converter.period(), from_time: from_time, maximum_periods: maximum_periods}
    }

    pub fn check(&self, time: Time) -> bool {
        time - self.from_time > (self.maximum_periods as f64) * self.period
    }
}

#[derive(Debug)]
pub struct MotionAtTime {	
	// Coefficients for time evolution of the system from one impact to the next 
	parameters: Parameters,
	impact_time: Time,
	cos_coefficient: Coefficient,
	sin_coefficient: Coefficient,
	long_excursion_checker: LongExcursionChecker
}

impl MotionAtTime {
    fn new(parameters: Parameters, impact: Impact) -> MotionAtTime {
        let cos_coefficient = parameters.obstacle_offset() - parameters.gamma() * (parameters.forcing_frequency()*impact.time()).cos();
        
        let sin_coefficient = -(parameters.coefficient_of_restitution() * impact.velocity()) + parameters.forcing_frequency() * parameters.gamma() * (parameters.forcing_frequency()*impact.time()).sin();

        return MotionAtTime{
            parameters: parameters, 
            impact_time: impact.time(), 
            cos_coefficient: cos_coefficient, 
            sin_coefficient: sin_coefficient, 
            long_excursion_checker: LongExcursionChecker::new(parameters.maximum_periods(), parameters.converter(), impact.time())}
    }

    pub fn state(&self, time: Time) -> StateOfMotion {
        let lambda = time - self.impact_time;

        let cos_lambda = lambda.cos();
        let sin_lambda = lambda.sin();

        StateOfMotion{time: time,
            displacement: self.cos_coefficient * cos_lambda + self.sin_coefficient * sin_lambda + 
                self.parameters.gamma() * (time * self.parameters.forcing_frequency()).cos(),
            velocity: self.sin_coefficient * cos_lambda - self.cos_coefficient * sin_lambda - 
                self.parameters.forcing_frequency() * self.parameters.gamma() * (time * self.parameters.forcing_frequency()).sin() }
    }

    pub fn constrained_state(&self, time: Time) -> StateOfMotion {
        self.state(time).constrain(self.parameters.obstacle_offset())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MotionGenerator {
    parameters: Parameters
}

impl MotionGenerator {
    fn new(parameters: Parameters) -> MotionGenerator {
        MotionGenerator{parameters: parameters}
    }

    pub fn generate(&self, impact: Impact) -> MotionAtTime {
        MotionAtTime::new(self.parameters, impact)
    }

    pub fn parameters(&self) -> Parameters {
        self.parameters
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SearchParameters {
	initial_step_size: Time,
	minimum_step_size: Time
}

impl SearchParameters {
    fn new(initial_step_size: Time, minimum_step_size: Time) -> SearchParameters {
        SearchParameters{initial_step_size: initial_step_size, minimum_step_size: minimum_step_size}
    }

    pub fn default() -> SearchParameters {
        return SearchParameters{initial_step_size: 0.1, minimum_step_size: 0.000001}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MotionBetweenImpacts {
    //
    // Generates a trajectory from one impact to the next
    //
	motion_generator: MotionGenerator,
    impact_generator: ImpactGenerator,
	sticking: Sticking,
	search: SearchParameters,
	offset: Distance
}

impl MotionBetweenImpacts {

    pub fn new(parameters: Parameters) -> MotionBetweenImpacts {
        let sticking = Sticking::new(parameters);

        MotionBetweenImpacts{motion_generator: MotionGenerator::new(parameters), 
            impact_generator: ImpactGenerator::new(parameters.converter()),
            sticking: sticking, search: SearchParameters::default(), 
            offset: parameters.obstacle_offset()}
    }

    pub fn motion(&self, impact: Impact) -> MotionAtTime {
        self.motion_generator.generate(impact)
    }

    pub fn next_impact(&self, impact: Impact) -> NextImpactResult {

        let mut result = NextImpactResult::new(&self, impact);

        result.found_impact = true;

        let mut step_size = self.search.initial_step_size;

        // NextImpactResult accounts for sticking in the initial impact
        let initial_state = result.motion.last().unwrap();

        let mut current_time = initial_state.time;

        let motion_model = self.motion_generator.generate(
            self.impact_generator.generate(current_time, initial_state.velocity)
        );

        while step_size.abs() > self.search.minimum_step_size && result.found_impact {
            current_time += step_size;

            let current_state = motion_model.state(current_time);

            // Update step size - this is the bisection search algorithm
            if current_state.displacement < self.offset {
                // only record the state if it is physical
                // (i.e. non-penetrating)
                result.grow(current_state);

                if step_size < 0.0 {
                    step_size *= -0.5;
                }
            } else if current_state.displacement > self.offset {
                if step_size > 0.0 {
                    step_size *= -0.5;
                }
            } else {
                result.grow(current_state);
                step_size = 0.0;
            }

            if motion_model.long_excursion_checker.check(current_time) {
                result.found_impact = false;
            }
        }
        
        result
    }

    pub fn generator(&self) -> MotionGenerator {
        self.motion_generator
    }

    pub fn sticking(&self) -> Sticking {
        self.sticking
    }
}

pub struct NextImpactResult {
	motion: Vec<StateOfMotion>,
	found_impact: bool
}

impl NextImpactResult {
    fn new(motion: &MotionBetweenImpacts, impact: Impact) -> NextImpactResult {

        let mut trajectory: Vec<StateOfMotion> = vec![];
        
        trajectory.push(StateOfMotion {time: impact.time(), displacement: motion.offset, velocity: impact.velocity()});
        
        let release_impact = motion.sticking.check_impact(impact);
        
        if release_impact.new_impact() {
            trajectory.push(StateOfMotion{
                time: release_impact.impact().time(), 
                displacement: motion.offset, 
                velocity: release_impact.impact().velocity()})
        }

        NextImpactResult{motion: trajectory, found_impact: false}
    }

    pub fn grow(&mut self, state: StateOfMotion) -> () {
        self.motion.push(state);
    }

    pub fn last(&self) -> StateOfMotion {
        *self.motion.last().unwrap()
    }

    pub fn found_impact(&self) -> bool {
        self.found_impact
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::impact::ImpactGenerator;

    #[test]
    fn test_motion_at_time() {
        let parameters = Parameters::new(2.8, 0.0, 0.8, 100).unwrap();

        let motion_generator = MotionGenerator::new(parameters);

        let impact_generator = ImpactGenerator::new(parameters.converter());

        let motion = motion_generator.generate(impact_generator.generate(0.0,1.0));

        let state = motion.state(0.0);

        assert_eq!(state.displacement, 0.0);
        assert_eq!(state.velocity, -0.8);
    }

    #[test]
    fn test_long_excursions() {
        let maximum_periods = 100u32;
        let converter = PhaseConverter::new(4.85).unwrap();
        let checker = LongExcursionChecker::new(maximum_periods, converter, 0.0);

        let good_time = (maximum_periods - 1) as Time * converter.period();
        let bad_time = (maximum_periods + 1) as Time * converter.period();

        assert!(!checker.check(good_time));
        assert!(checker.check(bad_time));
    }

    #[test]
    fn test_next_impact() {
        let parameters = Parameters::new(4.85, -0.1, 0.8, 100).unwrap();

        let motion_generator = MotionBetweenImpacts::new(parameters);

        let impact_generator = ImpactGenerator::new(parameters.converter());

        let impact_result = motion_generator.next_impact(impact_generator.generate(0.0, 0.0));

        assert!(impact_result.found_impact);
    }
}
//
// Time evolution of the system from one impact to the next
//
use super::parameters::Parameters as Parameters;
use super::forcing_phase::PhaseConverter as PhaseConverter;
use super::model_types::Phase as Phase;
use super::model_types::Time as Time;
use super::model_types::Distance as Distance;
use super::model_types::Velocity as Velocity;
use super::model_types::ParameterError as ParameterError;
use super::impact::Impact as Impact;
use super::impact::ImpactGenerator as ImpactGenerator;

pub struct StateOfMotion {
	// 	State and phase variables for the motion between impacts
	time: Time;
	displacement: Distance,
	velocity: Velocity
}

#[derive(Debug)]
pub struct LongExcursionChecker {
    converter: PhaseConverter,
    from_time: Time,
    maximum_periods: u32
}

#[derive(Debug)]
impl LongExcursionChecker {
    fn new(maximum_periods: u32, converter: PhaseConverter, from_time: Time) {
        LongExcursionChecker{converter: converter, from_time: from_time, maximum_periods: maximum_periods}
    }

    pub fn check(&self, time: Time) -> bool {
        time - self.from_time > float64(MaximumPeriods) * converter.get_period()
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
    fn new(parameters: Parameters, converter: PhaseConverter, impact: Impact) -> MotionAtTime {
        let cos_coefficient = parameters.obstacle_offset() - parameters.gamma() * parameters.forcing_frequency(impact.time()).cos();
        
        let sin_coefficient = -(parameters.coefficient_of_restitution() * impact.velocity()) + parameters.forcing_frequency() * parameters.gamma() * parameters.forcing_frequency(impact.time()).sin();

        return MotionAtTime{
            parameters: parameters, 
            impact_time: impact.Time, 
            cos_coefficient: cos_coefficient, 
            sin_coefficient: sin_coefficient, 
            long_excursion_checker: LongExcursionChecker(parameters.maximum_periods(), converter, impact.Time)}
    }

    pub fn state(time: Time) -> StateOfMotion {
        let lambda = time - motion.impact_time;

        let cos_lambda = lambda.cos();
        let sin_lambda = lambda.sin();

        StateOfMotion{time: time,
            displacement: motion.cos_coefficient * cos_lambda + motion.sin_coefficient * sin_lambda + 
                motion.parameters.gamma() * (time * motion.parameters.forcing_frequency()).cos(),
            velocity: motion.sin_coefficient * cos_lambda - motion.cos_coefficient * sin_lambda - 
                motion.parameters.forcing_frequency() * motion.parameters.gamma() * (time * motion.parameters.forcing_frequency()).sin() }
    }
}

#[derive(Debug)]
pub struct MotionGenerator {
    parameters: Parameters:
    converter: PhaseConverter
}

impl MotionGenerator {
    fn new(parameters: Parameters) -> MotionGenerator {
        MotionGenerator{parameters: parameters, converter: PhaseConverter::new(parameters.forcing_frequency()).unwrap()}
    }

    pub fn generate(&self, impact: Impact) -> MotionAtTime {
        MotionAtTime::new(self.parameters, self.converter, impact)
    }
}

#[derive(Debug)]
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

pub struct MotionBetweenImpacts {
    //
    // Generates a trajectory from one impact to the next
    //
	motion_generator: MotionGenerator,
	sticking: Sticking,
	search: SearchParameters,
	offset: Distance
}

impl MotionBetweenImpacts {

    fn new(parameters: Parameters) -> MotionBetweenImpacts {
        let sticking = Sticking::new(parameters).unwrap();

        MotionBetweenImpacts{motion_generator: MotionGenerator::new(parameters), sticking: sticking, search: SearchParameters::default(), offset: parameters.obstacle_offset()}
    }

    pub fn motion(&self, impact: Impact) -> MotionAtTime {
        self.motion_generator.generate(impact)
    }

    fn new_next_impact_result(&self, impact: Impact) -> NextImpactResult {

        let mut trajectory: Vec<StateOfMotion> = vec![];
        
        trajectory.push(trajectory, StateOfMotion {time: impact.time(), displacement: self.offset, velocity: impact.velocity()});
        
        let release_impact = self.sticking.check_impact(impact);
        
        if (release_impact.new_impact()) {
            trajectory = append(trajectory, StateOfMotion{
                time: release_impact.impact().time(), 
                displacement: motion.offset, 
                velocity: releaseImpact.impact().velocity()})
        }

        NextImpactResult{motion: trajectory, found_impact: false}
    }

    fn default_next_impact_result(&self, impact: Impact) -> NextImpactResult {
        self.new_next_impact_result(300, impact)
    }

    fn next_impact(&self, impact: Impact) -> NextImpactResult {

        let mut result = self.default_next_impact_result(impact);

        result.found_impact = true;

        let mut step_size = self.search.initial_step_size;

        let motion_model = self.motion_generator.generate(impact);

        let mut current_time = result.motion.last().unwrap().time;

        for math.Abs(step_size) > self.search.minimum_step_size && result.found_impact {
            current_time += step_size;

            let current_state = motion_model.state(current_time);

            // Update step size - this is the bisection search algorithm
            if current_state.displacement < self.offset {
                // only record the state if it is physical
                // (i.e. non-penetrating)
                result = result.grow(current_state);

                if step_size < 0.0 {
                    step_size *= -0.5;
                }
            } else if current_state.displacement > self.offset {
                if (step_size > 0) {
                    step_size *= -0.5;
                }
            } else {
                result = result.grow(current_state);
                step_size = 0;
            }

            if (motion_model.long_excursion(current_time)) {
                result.FoundImpact = false;
            }
        }
        
        result
    }

}

pub struct NextImpactResult {
	motion: Vec<StateOfMotion>,
	found_impact: bool
}

impl NextImpactResult {
    fn new() -> NewNextImpactResult {
        NextImpactResult{motion: vec![], found_impact: false}
    }

    pub fn grow(&mut self, state: StateOfMotion) -> () {
        self.motion.push(state);
    }
}

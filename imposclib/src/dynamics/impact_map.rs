use super::impact::Impact;
use super::motion::MotionBetweenImpacts as MotionBetweenImpacts;
use super::chatter::ChatterChecker as ChatterChecker;
use super::model_types::Time as Time;
use super::model_types::Phase as Phase;
use super::model_types::Velocity as Velocity;
use super::model_types::Coefficient as Coefficient;
use super::impact::ImpactGenerator as ImpactGenerator;
use super::parameters::Parameters as Parameters;
use super::forcing_phase::PhaseConverter as PhaseConverter;
use log::debug;

pub struct IterationResult 
{
	impacts: Vec<Impact>,

	long_excursions: bool
}

impl IterationResult {
    pub fn trajectory(&self) -> &Trajectory {&self.impacts}
    pub fn has_long_excursions(&self) -> bool {self.long_excursions}
}

pub struct ImpactResult 
{
	impact: Impact,
	found_impact: bool
}

type Trajectory = Vec<Impact>;

pub struct SingularitySetResult {
    singularity_set: Trajectory,
    dual: Trajectory
}

impl SingularitySetResult {
    pub fn singularity_set(&self) -> &Trajectory {
        &self.singularity_set
    }

    pub fn dual(&self) -> &Trajectory {
        &self.dual
    }
}

pub struct ImpactMap {
	
	// Transformation of the impact surface (an infinite half cylinder parametrised by phase and velocity)
	// which maps impacts to impacts
		
	motion: MotionBetweenImpacts,
	chatter_checker: ChatterChecker,
	generator: ImpactGenerator,
    coefficient_of_restitution: Coefficient
}

impl ImpactMap {
    pub fn new(parameters: Parameters) -> ImpactMap {
        let motion = MotionBetweenImpacts::new(parameters);

        ImpactMap{motion: motion, chatter_checker: ChatterChecker::default(parameters), 
        generator: ImpactGenerator::new(parameters.converter()), 
        coefficient_of_restitution: parameters.coefficient_of_restitution()}
    }

    pub fn generate_impact(&self, time: Time, velocity: Velocity) -> Impact {
        self.generator.generate(time, velocity)
    }

    // Apply the map to an impact
    pub fn apply(&self, impact: Impact) -> ImpactResult {
        debug!("Applying impact map to impact {:?}", impact);

        let trajectory = self.motion.next_impact(impact);

        let state_at_impact = trajectory.last();

        ImpactResult{impact: self.generate_impact(state_at_impact.time(), state_at_impact.velocity()), found_impact: trajectory.found_impact()}
    }

    // Iterate the map 
    pub fn iterate(&mut self, initial_impact: Impact, num_iterations: u32) -> IterationResult {
        debug!("Iterating from impact {:?}", initial_impact);

        let mut long_excursions = false;

        let mut trajectory = vec![initial_impact];

        for _ in 0..num_iterations {
            let next_impact = self.apply(*trajectory.last().unwrap());

            trajectory.push(next_impact.impact);

            if !next_impact.found_impact {
                long_excursions = true;
            }

            // Now check for chatter
            let chatter_result = self.chatter_checker.check(*trajectory.last().unwrap());

            if chatter_result.is_chatter() {
                trajectory.push(chatter_result.accumulation_impact());
            }
        }

        IterationResult{long_excursions: long_excursions, impacts: trajectory}
    }

    // Convenient overload
    pub fn iterate_from_point(&mut self, phi: Phase, v: Velocity, num_iterations: u32) -> IterationResult {
        let t = self.converter().time_into_cycle(phi);
        self.iterate(self.generate_impact(t, v), num_iterations)
    }

    pub fn converter(&self) -> PhaseConverter {
        return self.motion.generator().parameters().converter()
    }

    // Generate a singularity set
    pub fn singularity_set(&self, num_points: u32) -> SingularitySetResult {
        let num_points_to_use = std::cmp::max(1, num_points);

        let mut result = SingularitySetResult{singularity_set: vec![], dual: vec![]};

        let converter = self.converter();

        let mut starting_time = converter.period() * self.chatter_checker.sticking().phase_out();
        let ending_time = converter.period() * self.chatter_checker.sticking().phase_in();

        let delta_time = (ending_time - starting_time)/(num_points_to_use as f64);

        for _ in 0..num_points_to_use {
            let impact_result = self.apply(self.generate_impact(starting_time, 0.0));

            if impact_result.found_impact {
                result.dual.push(impact_result.impact);

                result.singularity_set.push(impact_result.impact.dual_impact(self.coefficient_of_restitution));
            }

            starting_time += delta_time;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_apply_always_returns() {
    //     let parameters = Parameters::new(4.85, -0.1, 0.8, 100).unwrap();

    //     let mapper = ImpactMap::new(parameters);

    //     let impact_result = mapper.apply(mapper.generate_impact(0.0, 0.0));

    //     assert!(impact_result.found_impact);
    // }
}

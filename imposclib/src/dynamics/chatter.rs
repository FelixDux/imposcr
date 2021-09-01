use super::parameters::Parameters as Parameters;
use super::model_types::Time as Time;
use super::model_types::Velocity as Velocity;
use super::motion::MotionBetweenImpacts as MotionBetweenImpacts;
use super::impact::Impact as Impact;
use super::sticking::Sticking as Sticking;


pub struct ChatterResult {
	is_chatter: bool,
	accumulation_impact: Impact
}

pub struct ChatterChecker<'a> {
	
	// Detects and numerically approximates 'Chatter', which is when an infinite sequence of impact.Impacts accumulates 
	// in a finite time on a 'sticking' impact. It is the analogue in this system to a real-world situation in 
	// which the mass judders against the stop. To handle it numerically it is necessary to detect when it is 
	// happening and then extrapolate forward to the accumulation point.
		
		velocity_threshold: Velocity,
		count_threshold: u32,
		sticking: &'a Sticking<'a>,
        parameters: &'a Parameters,
		can_chatter: bool,
		impact_count: u32
}

impl<'a> ChatterChecker<'a> {
    pub fn new(motion: &'a MotionBetweenImpacts, velocity_threshold: Velocity, count_threshold: u32) -> ChatterChecker<'a> {
        let parameters = motion.generator().parameters();

        let can_chatter = parameters.coefficient_of_restitution() < 1.0 && parameters.coefficient_of_restitution() >=0.0;

        ChatterChecker {
                velocity_threshold: velocity_threshold,
                count_threshold: count_threshold,
                impact_count: 0,
                can_chatter: can_chatter,
                sticking: motion.sticking(),
                parameters: motion.generator().parameters()
            }
    }

    fn accumulation_time(&self, impact: Impact) -> Time {

        if self.can_chatter { 
            return impact.time() - 2.0*impact.velocity() / (1.0-self.parameters.coefficient_of_restitution()) /
                ((impact.time() * self.parameters.forcing_frequency()).cos() - self.parameters.obstacle_offset());
        }

        impact.time()
    }

    pub fn check(&mut self, impact: Impact) -> ChatterResult {
        if self.can_chatter && impact.velocity() < self.velocity_threshold {
            self.impact_count += 1;
            if self.impact_count > self.count_threshold {
                self.impact_count = 0;
                let new_time = self.accumulation_time(impact);

                if self.sticking.time_sticks(new_time) {
                    return ChatterResult{is_chatter: true, accumulation_impact: self.sticking.generate(new_time)};
                }
            }
        }
        
        ChatterResult{is_chatter: false, accumulation_impact: impact}
    }

    pub fn default(motion: &'a MotionBetweenImpacts) -> ChatterChecker<'a> {
        ChatterChecker::new(motion, 0.05, 10)
    }
}
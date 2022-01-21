use super::parameters::Parameters as Parameters;
use super::forcing_phase::PhaseConverter as PhaseConverter;
use super::model_types::Phase as Phase;
use super::model_types::Time as Time;
use super::impact::Impact as Impact;
use super::impact::ImpactGenerator as ImpactGenerator;

pub struct ReleaseImpact {
	new_impact: bool,
	impact: Impact
}

impl ReleaseImpact {
    pub fn new_impact(&self) -> bool {
        self.new_impact
    }

    pub fn impact(&self) -> Impact {
        self.impact
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sticking {
	phase_in: Phase,
	phase_out: Phase,
	converter: PhaseConverter,
	generator: ImpactGenerator
}

impl Sticking {
    pub fn phase_in(&self) -> Phase {
        self.phase_in
    }

    pub fn phase_out(&self) -> Phase {
        self.phase_out
    }
}

impl Sticking {
    pub fn new(parameters: Parameters) -> Sticking {
    
        let converter = parameters.converter();

        let phase_in;
        let phase_out;

        if 1.0 <= parameters.obstacle_offset() {
            // No self
            phase_in = 0.0;
            phase_out = 0.0;
        } else if -1.0 >= parameters.obstacle_offset() || parameters.forcing_frequency() == 0.0 {
            // Sticking for all phases
            phase_in = 1.0;
            phase_out = 0.0;
        } else { 

            // (OK to divide by forcing frequency because zero case trapped above)
            let angle = parameters.obstacle_offset().acos();
            let phase1 = converter.time_to_phase(angle/parameters.forcing_frequency());
            let phase2 = 1.0 - phase1;

            if angle.sin() < 0.0 {
                phase_in = phase1;
                phase_out = phase2;
            } else {
                phase_in = phase2;
                phase_out = phase1;
            }
        }

        Sticking{phase_in: phase_in, phase_out: phase_out, converter: converter, generator: ImpactGenerator::new(converter)}
    }

    pub fn never(&self) -> bool {
    	self.phase_in == self.phase_out
    }

    pub fn always(&self) -> bool {
    	return self.phase_in == 1.0 && self.phase_out == 0.0
    }

    pub fn phase_sticks(&self, phase: Phase) -> bool {
    	if self.never() {
            return false;
        }

    	if self.always() {
            return true;
        }

    	phase < self.phase_out || phase >= self.phase_in
    }

    pub fn time_sticks(&self, time: Time) -> bool {
    	self.phase_sticks(self.converter.time_to_phase(time))
    }

    pub fn release_time(&self, time: Time) -> Time {
    	return self.converter.forward_to_phase(time, self.phase_out)
    }

    pub fn check_impact(&self, impact: Impact) -> ReleaseImpact {

    	if impact.velocity() == 0.0 && self.phase_sticks(impact.phase()) && !self.always() {
    		return ReleaseImpact{new_impact: true, impact: self.generator.generate(self.release_time(impact.time()), 0.0)}
    	} else {
    		return ReleaseImpact{new_impact: false, impact: impact}
    	}
    }

	pub fn generate(&self, impact_time: Time) -> Impact  {
        self.generator.generate(impact_time, 0.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sticking_region() -> () {
        let frequencies = vec![2.8, 3.7, 4.0, 4.85];
    
        let r = 0.8;
    
        let offsets = vec![0.0, -0.1];
    
        for frequency in frequencies.iter() {
            for offset in offsets.iter() {
                let params = Parameters::new(*frequency, *offset, r, 100).unwrap();
        
                let sticking = Sticking::new(params);
        
                let impact_time = 0.0;
        
                assert!(sticking.time_sticks(impact_time));
        
                assert!(sticking.release_time(impact_time) > impact_time);
        
                assert!(sticking.phase_in >= sticking.phase_out);    
            }
        }
    }
}
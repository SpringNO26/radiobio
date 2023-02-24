/* ---------------------------- External imports ---------------------------- */
use anyhow::Result;
use anyhow::bail;

/* ---------------------------- Internal imports ---------------------------- */


/* -------------------------------------------------------------------------- */
/*                         FUNCTION/STRUCT DEFINITIONS                        */
/* -------------------------------------------------------------------------- */
#[derive(Debug)]
pub enum Beam {
    Constant(ParticleBeam),
    Pulsed(ParticleBeam)
}
impl Beam {
    pub fn new_constant(particle:String, dose_rate:f64)
    -> Result<Self> {
        Ok(Beam::Constant(ParticleBeam::new(
            particle,
            dose_rate,
            TimeStructure::new_constant(),
        )))
    }

    pub fn new_pulsed(particle:String, dose_rate:f64, period:f64, on_time:f64)
    -> Result<Self> {
        if on_time > period {
            bail!("While constructing Beam, found on_time ({}) > period ({})",
                  on_time,
                  period
            );
        }
        Ok(Beam::Pulsed(ParticleBeam::new(
            particle,
            dose_rate,
            TimeStructure::new_pulsed(period, on_time),
        )))
    }

    pub fn as_particle_beam(&self) -> &ParticleBeam {
        match self {
            Beam::Constant(beam) => beam,
            Beam::Pulsed(beam) => beam,
        }
    }
    pub fn as_mut_particle_beam(&mut self) -> &mut ParticleBeam {
        match self {
            Beam::Constant(beam) => beam,
            Beam::Pulsed(beam) => beam,
        }
    }
    pub fn particle(&self) -> &String {
        self.as_particle_beam().particle()
    }
    pub fn average_dose_rate(&self) -> f64 {self.as_particle_beam().average_dose_rate()}
    pub fn peak_dose_rate(&self) -> f64 {
        self.as_particle_beam().peak_dose_rate()
    }

}

impl IsTimed for Beam {
    fn at(&self, time:f64) -> TimeMessage {
        TimeMessage {
            time: time,
            current_dose_rate: self.as_particle_beam().peak_dose_rate(),
        }
    }

    fn set_structure(&mut self, ts:TimeStructure) {
        self.as_mut_particle_beam().set_structure(ts)
    }
    fn get_structure(&self) -> &TimeStructure {
        self.as_particle_beam().get_structure()
    }
}

#[derive(Debug)]
pub enum TimeState {
    IsON,
    IsOFF
}

#[derive(Clone, Debug)]
pub struct TimeStructure {
    period: f64,
    on_time: f64,
}

impl TimeStructure {
    pub fn new_constant() -> Self {
        Self { period:f64::MAX, on_time:f64::MAX}
    }
    pub fn new_pulsed(period:f64, on_time:f64) -> Self {
        Self { period, on_time }
    }
    pub fn state_at(&self, time:f64) -> TimeState {
        match (time%self.period) <= self.on_time {
            true => TimeState::IsON,
            false => TimeState::IsOFF,
        }
    }
    pub fn duty_cycle(&self) -> f64 {
        self.on_time / self.period // return 'inf' if period = 0 (no error)
    }
}

#[allow(non_snake_case)]
pub trait IsTimed{

    fn set_structure(&mut self, ts:TimeStructure);
    fn get_structure(&self) -> &TimeStructure;
    fn at(&self, time:f64) -> TimeMessage;
}

#[derive(Clone, Debug)]
pub struct ParticleBeam {
    particle: String,
    dose_rate: f64, // average dose rate over 1 period
    time_struct: TimeStructure,
}
impl ParticleBeam {
    pub fn new(particle:String, dose_rate:f64, time_struct:TimeStructure) -> Self {
        Self{particle, dose_rate, time_struct}
    }

    pub fn particle(&self) -> &String {&self.particle}
    pub fn average_dose_rate(&self) -> f64 {self.dose_rate}
    pub fn peak_dose_rate(&self) -> f64 {
        self.dose_rate / self.time_struct.duty_cycle()
    }
}
impl IsTimed for ParticleBeam {
    fn at(&self, time:f64) -> TimeMessage {
        TimeMessage {
            time: time,
            current_dose_rate: self.peak_dose_rate(),
        }
    }

    fn set_structure(&mut self, ts:TimeStructure) {
        self.time_struct = ts; // Move
    }
    fn get_structure(&self) -> &TimeStructure {
        &(self.time_struct)
    }
}

pub struct TimeMessage {
    time: f64,
    current_dose_rate: f64,
}

#[allow(non_snake_case)]
impl TimeMessage {
    pub fn is_ON(&self) -> bool { self.current_dose_rate > 0_f64 }
    pub fn dose_rate(&self) -> f64 { self.current_dose_rate }
    pub fn time(&self) -> f64 { self.time }
}


/* -------------------------------------------------------------------------- */
/*                                   TESTING                                  */
/* -------------------------------------------------------------------------- */
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}

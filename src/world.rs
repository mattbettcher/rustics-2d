use rayon::prelude::*;
use {Body, Vector2};

/// Container for the simulation.
pub struct World {
    pub bodies: Vec<Body>,
    /// Gravity in this `World`. The default gravity is (0, -9.81).
    pub gravity: Vector2,

    pub allow_deactivation: bool,

    pub angular_damping_factor: f32,
    pub linear_damping_factor: f32,
    pub accumulated_time: f64,
}

impl World {
    pub fn new() -> Self {
        World {
            bodies: vec![],
            gravity: Vector2::new(0.0, -9.81),
            allow_deactivation: false,
            angular_damping_factor: 0.95,
            linear_damping_factor: 0.95,
            accumulated_time: 0.0,
        }
    }

    /// Integrates the whole world several fixed timesteps further in time.
    pub fn step_for(&mut self, time: f64, timestep: f32, max_steps: usize) {
        let mut steps_taken = 0;
        self.accumulated_time += time;

        while self.accumulated_time > timestep as f64 {
            self.step(timestep);

            self.accumulated_time -= timestep as f64;
            steps_taken += 1;

            if steps_taken > max_steps {
                self.accumulated_time = 0.0;
                break;
            }
        }

        //println!("Steps taken: {:?}", steps_taken);
    }

    /// Integrates the whole world a single timestep further in time.
    ///
    /// # Notes
    ///
    /// The physics simulation shouldn't run slower than 60fps. (1/60)
    pub fn step(&mut self, timestep: f32) {
        // collision_system.detect()

        self.integrate_forces(timestep);
        self.integrate(timestep);
    }

    fn integrate_forces(&mut self, timestep: f32) {
        let g = self.gravity;   // to avoid self borrow
        self.bodies.par_iter_mut()
            .for_each(|b| {
                if !b.is_static && b.is_active {
                    if b.affected_by_gravity { b.force += g; }
                    b.linear_velocity += b.force * b.inv_mass * timestep;
                    b.angular_velocity += b.torque * b.inv_inertia * timestep;
                }
                b.force = Vector2::new(0.0, 0.0);
                b.torque = 0.0;
            });
    }

    fn integrate(&mut self, timestep: f32) {
        let ldf = self.linear_damping_factor;   // to avoid self borrow
        let adf = self.angular_damping_factor;
        self.bodies.par_iter_mut()
            .for_each(|b| {
                if !b.is_static && b.is_active {
                    b.position += b.linear_velocity * timestep;
                    b.orientation += b.angular_velocity * timestep;
                    b.linear_velocity = b.linear_velocity * ldf;
                    b.angular_velocity = b.angular_velocity * adf;
                    b.update();
                }
            });
    }

    /// Every computation `step` the angular and linear velocity
    /// of a `Body` gets multiplied by this value.
    /// The default values are 0.85.
    ///
    /// # Panics
    ///
    /// Both damping factors must be between 0.0 and 1.0.
    pub fn set_damping_factors(&mut self, linear: f32, angular: f32) {
        assert!(
            linear < 0.0 || linear > 1.0,
            "Linear damping factor has to be between 0.0 and 1.0"
        );
        assert!(
            angular < 0.0 || angular > 1.0,
            "Angular damping factor has to be between 0.0 and 1.0"
        );

        self.linear_damping_factor = linear;
        self.angular_damping_factor = angular;
    }
}

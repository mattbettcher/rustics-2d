use {BoundingBox, V2};

///
pub struct Body {
    /// Position.
    pub position: V2,
    /// Orientation.
    pub orientation: f32,
    ///
    pub inertia: f32,
    ///
    pub linear_velocity: V2,

    pub angular_velocity: f32,

    pub force: V2,

    pub torque: f32,

    pub mass: f32,

    pub is_static: bool,

    pub is_active: bool,

    pub affected_by_gravity: bool,

    pub bbox: BoundingBox,

    pub inv_mass: f32,
    pub inv_inertia: f32,
    pub inv_orientation: f32,
}

impl Body {
    pub fn new(position: V2) -> Self {
        Body {
            position: position,
            orientation: 0.0,
            inertia: 0.0,
            linear_velocity: V2::ZERO,
            angular_velocity: 0.0,
            force: V2::ZERO,
            torque: 0.0,
            mass: 1.0,
            is_static: false,
            is_active: true,
            affected_by_gravity: true,
            bbox: BoundingBox::MIN,
            inv_mass: 1.0,
            inv_inertia: 0.0,
            inv_orientation: 0.0,
        }
    }


    /// Applies an impulse on the specific position. Changing linear
    /// and angular velocity.
    pub fn apply_impulse(&mut self, impulse: &V2, relative_position: Option<V2>) {
        assert!(!self.is_static, "Can't apply an impulse to a static body.");

        self.linear_velocity += *impulse * self.inv_mass;
        if let Some(rel_pos) = relative_position {
            self.angular_velocity += self.inv_inertia * V2::cross(&rel_pos, &impulse);
        }
    }

    pub fn add_force(&mut self, force: &V2, relative_position: Option<V2>) {
        self.force += *force;
        if let Some(rel_pos) = relative_position {
            self.torque += V2::cross(&rel_pos, force);
        }
    }

    pub fn update(&mut self) {
        self.inv_orientation = -self.orientation;
        // todo - bounding box update -- needs shape???
    }
}

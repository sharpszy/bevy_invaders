use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{WinSize, BASE_SPEED, FORMATION_MEMBERS_MAX};

/// Componet - Enemy Formation (per enemy)
#[derive(Clone, Component)]
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub pivot: (f32, f32),
    pub speed: f32,
    pub angle: f32, // change per tick
}

/// Resourece - Fromation Maker
#[derive(Default, Resource)]
pub struct FormationMaker {
    current_template: Option<Formation>,
    current_members: u32,
}

/// Formation factory implemention
impl FormationMaker {
    pub fn make(&mut self, win_size: &WinSize) -> Formation {
        match (
            &self.current_template,
            self.current_members >= FORMATION_MEMBERS_MAX,
        ) {
            // if has current template and still within max memebers
            (Some(tmpl), false) => {
                self.current_members += 1;
                tmpl.clone()
            }
            (None, _) | (_, true) => {
                let mut rng = thread_rng();

                // compute the start x/y
                let w_span = win_size.w / 2. + 100.;
                let h_span = win_size.h / 2. + 100.;
                let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
                let y = rng.gen_range(-h_span..h_span);
                let start = (x, y);

                // compute the pivot x/y
                let w_span = win_size.w / 4.;
                let h_span = win_size.h / 3. + 50.;
                let pivot = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));

                // compute the radius
                let radius = (rng.gen_range(80.0..150.0), 100.);

                // compute the start angle
                let angle = (y - pivot.1).atan2(x - pivot.0);

                // speed (fixed for now)
                let speed = BASE_SPEED;

                let formation = Formation {
                    start,
                    radius: radius,
                    pivot,
                    speed,
                    angle,
                };

                // store as template
                self.current_template = Some(formation.clone());
                // reset the memebers to 1
                self.current_members = 1;

                formation
            }
        }
    }
}

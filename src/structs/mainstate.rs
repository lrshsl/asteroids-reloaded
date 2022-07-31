use crate::{
    Ship, Asteroid,
    GameParams,
    KeyCode, is_key_down,
};

use rand::{self, Rng};


pub struct MainState {
    score: u32,
    ship: Ship,
    asteroids: Vec<Asteroid>,
    params: GameParams,
}

impl MainState {
    pub fn new(params: GameParams) -> Self {
        let ship = Ship::new(params.ship.clone());
        let asteroids = Vec::new();
        Self {
            score: 0,
            ship,
            asteroids,
            params,
        }
    }

    pub fn draw(&self) {
        self.ship.draw_self();
        for a in self.asteroids.iter() {
            a.draw_self();
        }
    }

    pub fn update(&mut self) {
        let rnd: f32 = rand::thread_rng().gen();
        if rnd > self.params.asteroid.spawning_rate {
            self.asteroids.push(Asteroid::new_random(self.params.asteroid.clone()));
        }
        for ast in self.asteroids.iter_mut() {
            ast.update_self();
        }
        for ast in self.asteroids.iter() {
            for pot_collision_obj in self.asteroids.iter() {
                if ast.is_overlapping(pot_collision_obj) {
                    ast.collide(pot_collision_obj);
                    pot_collision_obj.collide(ast);
                }
            }
        }
        self.ship.update_self();
    }

    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::H) {
            self.ship.turn(-self.ship.turn_speed);
        } else if is_key_down(KeyCode::L) {
            self.ship.turn(self.ship.turn_speed);
        }
        if is_key_down(KeyCode::K) {
            self.ship.accelerate(1.)
        } else if is_key_down(KeyCode::J) {
            self.ship.accelerate(-1.)
        }
    }
}

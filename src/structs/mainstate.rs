use crate::{constants::MAX_ASTEROIDS, is_key_down, Asteroid, GameParams, KeyCode, Ship};

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
        // spawn new asteroids
        let rnd: f32 = rand::thread_rng().gen();
        if self.asteroids.len() < MAX_ASTEROIDS && rnd > self.params.asteroid.spawning_rate {
            self.asteroids
                .push(Asteroid::new_random(self.params.asteroid.clone()));
        }

        // update
        for ast in self.asteroids.iter_mut() {
            ast.update_self();
        }
        self.ship.update_self();

        // collide

        for ast in self.get_collided_asteroids().iter() {
            self.score += 1;
            self.split_ast(ast);
            self.ship.collide(ast);
        }
    }

    fn get_collided_asteroids(&self) -> Vec<Asteroid> {
        let mut collided_asteroids = Vec::new();
        for ast in self.asteroids.iter() {
            if self.ship.is_overlapping(ast) {
                collided_asteroids.push((*ast).clone());
                println!("len: {}", collided_asteroids.len());
            }
        }
        collided_asteroids
    }

    fn split_ast(&mut self, ast: &Asteroid) {
        self.asteroids.retain(|a| *a != *ast);
        self.asteroids.push(ast.get_child());
        self.asteroids.push(ast.get_child());
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

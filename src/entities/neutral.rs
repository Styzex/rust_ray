extern crate sdl2;

use crate::rrm::MAP_CUBE_SIZE as SIZE;
use crate::utilities::opengl::setup_viewport;
use dashmap::*;
use glu_sys::*;
use std::f32::consts::PI;
use std::*;

pub struct NeutralEntities<'a> {
    pub list: DashMap<&'a str, &'a mut NeutralEntity>,
}

impl<'a> NeutralEntities<'a> {
    pub fn new() -> Self {
        NeutralEntities {
            list: DashMap::new(),
        }
    }

    pub fn add(&mut self, name: &'a str, value: &'a mut NeutralEntity) {
        self.list.insert(name, value);
    }

    pub fn update_all(&mut self, player_x: f32, player_y: f32) {
        self.list.iter_mut().for_each(|entity| {
            let mut entity = entity;
            entity.follow_player(player_x, player_y);
        });
    }

    pub fn draw_all(&mut self, screen_width: i32, screen_height: i32) {
        self.list.iter_mut().for_each(|entity| {
            let mut entity = entity;
            entity.draw_neutral_entity(screen_width, screen_height);
        });
    }

    pub fn print(&self) {
        println!("{:?}", self.list);
    }
}

#[derive(Debug)]
pub struct NeutralEntity {
    pub x: f32,
    pub y: f32,
    pub angle: f32,     // Rotation angle in radians
    pub speed: f32,     // Movement speed
    pub health: f32,    // Health points
    pub is_alive: bool, // Living state
    pub sprite_id: u32, // ID for sprite rendering
}

impl PartialEq for NeutralEntity {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f32::EPSILON
            && (self.y - other.y).abs() < f32::EPSILON
            && (self.angle - other.angle).abs() < f32::EPSILON
            && (self.speed - other.speed).abs() < f32::EPSILON
            && (self.health - other.health).abs() < f32::EPSILON
            && self.is_alive == other.is_alive
            && self.sprite_id == other.sprite_id
    }
}

impl Eq for NeutralEntity {}

impl hash::Hash for NeutralEntity {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let x = (self.x * 1000.0) as i32;
        let y = (self.y * 1000.0) as i32;
        let angle = (self.angle * 1000.0) as i32;
        let speed = (self.speed * 1000.0) as i32;
        let health = (self.health * 1000.0) as i32;
        x.hash(state);
        y.hash(state);
        angle.hash(state);
        speed.hash(state);
        health.hash(state);
        self.is_alive.hash(state);
        self.sprite_id.hash(state);
    }
}

impl NeutralEntity {
    pub fn new(screen_width: i32, screen_height: i32) -> Result<Self, String> {
        unsafe {
            // Create the x and y values directly
            let x = screen_width as f32 / SIZE * 4.0;
            let y = screen_height as f32 / SIZE * 4.0;
            let angle: f32 = PI / 3.0;
            let speed: f32 = 0.5;
            let health: f32 = 20.0;
            let is_alive: bool = true;
            let sprite_id: u32 = 0;

            Ok(Self {
                x,
                y,
                angle,
                speed,
                health,
                is_alive,
                sprite_id,
            })
        }
    }

    pub fn follow_player(&mut self, player_x: f32, player_y: f32) {
        // Calculate direction vector
        let dx = player_x - self.x;
        let dy = player_y - self.y;

        self.speed = 0.5;

        // Calculate distance to player
        let distance = (dx * dx + dy * dy).sqrt();

        // Only move if the enemy is not already at the player's position
        if distance > 0.1 {
            // Small threshold to avoid jittering
            // Normalize the direction vector
            let direction_x = dx / distance;
            let direction_y = dy / distance;

            // Move towards player with constant speed
            self.x += direction_x * self.speed;
            self.y += direction_y * self.speed;
        }
    }

    pub fn draw_neutral_entity(&mut self, screen_width: i32, screen_height: i32) {
        let entity_x = self.x as i32;
        let entity_y = self.y as i32;

        unsafe {
            setup_viewport(screen_width, screen_height);

            glColor3f(0.0, 1.0, 0.0);
            glPointSize(16.0);
            glBegin(GL_POINTS);
            glVertex2i(entity_x, entity_y);
            glEnd();

            glLineWidth(3.00);
            glBegin(GL_LINES);
            glVertex2i(entity_x, entity_y);
            glEnd();
        }
    }
}

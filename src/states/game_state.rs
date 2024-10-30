use crate::rendering::{render_3d, TextRenderer};
use glu_sys::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;
use std::*;

#[derive(PartialEq)]
enum MenuItem {
    Play,
    Settings,
    Exit,
}

pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Settings,
    Transition,
}

pub struct GameStateManager {
    menu_selection: MenuItem,
    current_state: GameState,
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    screen_width: i32,
    screen_height: i32,
    mouse_x: f32,
    mouse_y: f32,
    ttf_context: Sdl2TtfContext, // Added TTF context as a field
}

impl GameStateManager {
    pub fn new(
        screen_width: i32,
        screen_height: i32,
        player_x: f32,
        player_y: f32,
        player_angle: f32,
        mouse_x: f32,
        mouse_y: f32,
    ) -> Result<Self, String> {
        // Initialize TTF context
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        Ok(Self {
            menu_selection: MenuItem::Play,
            current_state: GameState::Playing,
            player_x,
            player_y,
            player_angle,
            screen_width,
            screen_height,
            mouse_x,
            mouse_y,
            ttf_context,
        })
    }

    pub fn get_current_state(&self) -> &GameState {
        &self.current_state
    }

    pub fn set_state(&mut self, new_state: GameState) {
        self.current_state = new_state;
    }

    pub fn update(&mut self, event: &Event) {
        match self.current_state {
            GameState::MainMenu => self.update_main_menu(event),
            GameState::Playing => self.update_playing(event),
            GameState::Paused => self.update_paused(event),
            GameState::GameOver => self.update_game_over(event),
            GameState::Settings => self.update_settings(event),
            GameState::Transition => self.update_transition(event),
        }
    }

    pub fn render(&mut self) {
        match self.current_state {
            GameState::MainMenu => self.render_main_menu(),
            GameState::Playing => self.render_playing(),
            GameState::Paused => self.render_paused(),
            GameState::GameOver => self.render_game_over(),
            GameState::Settings => self.render_settings(),
            GameState::Transition => self.render_transition(),
        }
    }

    fn update_main_menu(&mut self, event: &Event) {}

    fn update_playing(&mut self, event: &Event) {
        match event {
            Event::MouseMotion { x, y, .. } => {
                self.mouse_x = *x as f32;
                self.mouse_y = *y as f32;

                let delta_x = self.mouse_x - self.player_x;
                let delta_y = self.mouse_y - self.player_y;
                self.player_angle = delta_y.atan2(delta_x);
            }
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                self.player_x += self.player_angle.cos() * 2.0;
                self.player_y += self.player_angle.sin() * 2.0;
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                self.player_x -= self.player_angle.cos() * 2.0;
                self.player_y -= self.player_angle.sin() * 2.0;
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                self.player_x += self.player_angle.sin() * 2.0;
                self.player_y += self.player_angle.cos() * 2.0;
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                self.player_x -= self.player_angle.sin() * 2.0;
                self.player_y -= self.player_angle.cos() * 2.0;
            }
            _ => {}
        }
    }

    fn update_paused(&mut self, event: &Event) {
        // Implement paused state update logic
    }

    fn update_game_over(&mut self, event: &Event) {
        // Implement game over state update logic
    }

    fn update_settings(&mut self, event: &Event) {
        // Implement settings state update logic
    }

    fn update_transition(&mut self, event: &Event) {
        // Implement transition state update logic
    }

    fn render_main_menu(&self) {
        // Clear the screen with a dark background
        unsafe {
            glClearColor(0.1, 0.1, 0.1, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);
        }

        let font = match self.ttf_context.load_font("assets/fonts/Arial.ttf", 48) {
            Ok(font) => font,
            Err(e) => {
                eprintln!("Failed to load font: {}", e);
                return;
            }
        };

        // Render title
        if let Ok(title_surface) = font
            .render("My Awesome Game")
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())
        {
            unsafe {
                glMatrixMode(GL_MODELVIEW);
                glLoadIdentity();
                glTranslated(self.screen_width as f64 / 2.0, 100.0, 0.0);
            }
            TextRenderer::render(&title_surface);
        }

        let menu_items = [
            ("Play", MenuItem::Play),
            ("Settings", MenuItem::Settings),
            ("Exit", MenuItem::Exit),
        ];

        for (i, (text, item)) in menu_items.iter().enumerate() {
            let color = if self.menu_selection == *item {
                Color::RGB(255, 255, 0) // Yellow for selected item
            } else {
                Color::RGB(200, 200, 200) // Grey for unselected items
            };

            if let Ok(item_surface) = font.render(text).blended(color).map_err(|e| e.to_string()) {
                unsafe {
                    glMatrixMode(GL_MODELVIEW);
                    glLoadIdentity();
                    glTranslated(
                        self.screen_width as f64 / 2.0,
                        250.0 + (i as f64 * 60.0),
                        0.0,
                    );
                }
                TextRenderer::render(&item_surface);
            }
        }
    }

    fn render_playing(&self) {
        render_3d(
            self.player_x,
            self.player_y,
            self.player_angle,
            self.screen_width,
            self.screen_height,
        );
    }

    fn render_paused(&self) {
        // Implement paused state rendering
    }

    fn render_game_over(&self) {
        // Implement game over state rendering
    }

    fn render_settings(&self) {
        // Implement settings state rendering
    }

    fn render_transition(&self) {
        // Implement transition state rendering
    }
}

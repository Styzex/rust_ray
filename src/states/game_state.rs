extern crate sdl2;

use crate::rendering::{render_3d, TextRenderer};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::*;

/// Represents the different items in the main menu
#[derive(PartialEq)]
enum MenuItem {
    Play,
    Settings,
    Exit,
}

#[derive(PartialEq)]
enum PauseItem {
    Continue,
    Settings,
    RTTitle,
}

/// Represents the different states of the game
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Settings,
    Transition,
}

/// Manages the game state and handles rendering and updating of different game states
pub struct GameStateManager {
    menu_selection: MenuItem,
    pause_selection: PauseItem,
    current_state: GameState,
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    screen_width: i32,
    screen_height: i32,
    mouse_x: f32,
    mouse_y: f32,
}

impl GameStateManager {
    /// Creates a new GameStateManager
    ///
    /// # Arguments
    ///
    /// * `screen_width` - The width of the game screen
    /// * `screen_height` - The height of the game screen
    /// * `player_x` - The initial x-coordinate of the player
    /// * `player_y` - The initial y-coordinate of the player
    /// * `player_angle` - The initial angle of the player
    /// * `mouse_x` - The initial x-coordinate of the mouse
    /// * `mouse_y` - The initial y-coordinate of the mouse
    ///
    /// # Returns
    ///
    /// A Result containing the new GameStateManager or an error string
    pub fn new(
        screen_width: i32,
        screen_height: i32,
        player_x: f32,
        player_y: f32,
        player_angle: f32,
        mouse_x: f32,
        mouse_y: f32,
    ) -> Result<Self, String> {
        Ok(Self {
            menu_selection: MenuItem::Play,
            pause_selection: PauseItem::Continue,
            current_state: GameState::MainMenu,
            player_x,
            player_y,
            player_angle,
            screen_width,
            screen_height,
            mouse_x,
            mouse_y,
        })
    }

    pub fn get_current_state(&self) -> &GameState {
        &self.current_state
    }

    pub fn set_state(&mut self, new_state: GameState) {
        self.current_state = new_state;
    }

    pub fn update(&mut self, event: &Event) -> bool {
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

    fn update_main_menu(&mut self, event: &Event) -> bool {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                &Keycode::W | &Keycode::Up => {
                    self.menu_selection = match self.menu_selection {
                        MenuItem::Play => MenuItem::Exit,
                        MenuItem::Settings => MenuItem::Play,
                        MenuItem::Exit => MenuItem::Settings,
                    };
                }
                &Keycode::S | &Keycode::Down => {
                    self.menu_selection = match self.menu_selection {
                        MenuItem::Play => MenuItem::Settings,
                        MenuItem::Settings => MenuItem::Exit,
                        MenuItem::Exit => MenuItem::Play,
                    };
                }
                &Keycode::SPACE | &Keycode::RETURN => match self.menu_selection {
                    MenuItem::Play => {
                        self.set_state(GameState::Playing);
                    }
                    MenuItem::Settings => {
                        self.set_state(GameState::Settings);
                    }
                    MenuItem::Exit => {
                        return true;
                    }
                },
                _ => {}
            },
            _ => {}
        }
        false
    }

    fn update_playing(&mut self, event: &Event) -> bool {
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
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                self.set_state(GameState::Paused);
            }
            _ => {}
        }
        false
    }

    fn update_paused(&mut self, event: &Event) -> bool {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                &Keycode::W | &Keycode::Up => {
                    self.pause_selection = match self.pause_selection {
                        PauseItem::Continue => PauseItem::RTTitle,
                        PauseItem::Settings => PauseItem::Continue,
                        PauseItem::RTTitle => PauseItem::Settings,
                    };
                }
                &Keycode::S | &Keycode::Down => {
                    self.pause_selection = match self.pause_selection {
                        PauseItem::Continue => PauseItem::Settings,
                        PauseItem::Settings => PauseItem::RTTitle,
                        PauseItem::RTTitle => PauseItem::Continue,
                    };
                }
                &Keycode::SPACE | &Keycode::RETURN => match self.pause_selection {
                    PauseItem::Continue => {
                        self.set_state(GameState::Playing);
                    }
                    PauseItem::Settings => {
                        self.set_state(GameState::Settings);
                    }
                    PauseItem::RTTitle => {
                        self.set_state(GameState::MainMenu);
                    }
                },
                _ => {}
            },
            _ => {}
        }
        false
    }

    fn update_game_over(&mut self, event: &Event) -> bool {
        // Implement game over state update logic
        false
    }

    fn update_settings(&mut self, event: &Event) -> bool {
        // Implement settings state update logic
        false
    }

    fn update_transition(&mut self, event: &Event) -> bool {
        // Implement transition state update logic
        false
    }

    fn render_main_menu(&mut self) {
        let title_renderer = TextRenderer::new("./assets/font/Mario.ttf", Color::RGB(255, 255, 0))
            .expect("Failed to load font");
        title_renderer.render_text(
            self.screen_width as f32 / 8.0 - 50.0,
            50.0,
            "rust ray",
            64.0,
        );

        let menu_items = [
            ("Play", MenuItem::Play),
            ("Settings", MenuItem::Settings),
            ("Exit", MenuItem::Exit),
        ];

        let menu_renderer = TextRenderer::new("./assets/font/Mario.ttf", Color::RGB(204, 204, 204))
            .expect("Failed to load font");

        for (i, (text, item)) in menu_items.iter().enumerate() {
            let mut selected_renderer =
                TextRenderer::new("./assets/font/Mario.ttf", Color::RGB(255, 255, 0))
                    .expect("Failed to load font");

            let current_renderer = if self.menu_selection == *item {
                &mut selected_renderer
            } else {
                &menu_renderer
            };

            current_renderer.render_text(
                self.screen_width as f32 / 8.0 - 50.0,
                250.0 + (i as f32 * 60.0),
                text,
                24.0,
            );
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
        render_3d(
            self.player_x,
            self.player_y,
            self.player_angle,
            self.screen_width,
            self.screen_height,
        );

        let title_renderer = TextRenderer::new("./assets/font/Mario.ttf", Color::RGB(255, 255, 0))
            .expect("Failed to load font");
        let title_text = "Paused";
        title_renderer.render_centered_text(self.screen_width as f32, 50.0, title_text, 64.0);

        let pause_items = [
            ("Continue", PauseItem::Continue),
            ("Settings", PauseItem::Settings),
            ("Return to title", PauseItem::RTTitle),
        ];

        let pause_renderer =
            TextRenderer::new("./assets/font/Mario.ttf", Color::RGB(204, 204, 204))
                .expect("Failed to load font");

        for (i, (text, item)) in pause_items.iter().enumerate() {
            let mut selected_renderer =
                TextRenderer::new("./assets/font/Mario.ttf", Color::RGB(255, 255, 0))
                    .expect("Failed to load font");

            let current_renderer = if self.pause_selection == *item {
                &mut selected_renderer
            } else {
                &pause_renderer
            };

            current_renderer.render_centered_text(
                self.screen_width as f32,
                250.0 + (i as f32 * 60.0),
                text,
                24.0,
            );
        }
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

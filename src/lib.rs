#![no_std]

mod wasm4;
use core::arch::wasm32;
use wasm4::*;

pub struct Game {
    cat: Cat,
    frame_count: u32,
}

impl Game {
    pub const fn new() -> Self {
        Game {
            cat: Cat::new(),
            frame_count: 0,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
    }

    fn input(&mut self) {
        let gamepad: u8 = get_gamepad();

        let mut velocity: (i8, i8) = (0, 0);
        if gamepad & BUTTON_LEFT != 0 {
            velocity = (-1, 0);
        } else if gamepad & BUTTON_RIGHT != 0 {
            velocity = (1, 0);
        } else if gamepad & BUTTON_UP != 0 {
            velocity = (0, -1);
        } else if gamepad & BUTTON_DOWN != 0 {
            velocity = (0, 1);
        }
        self.cat.set_velocity(velocity);
    }
}

pub struct Cat {
    sprite: [u8; 256],
    state: u8,
    position: (usize, usize),
    velocity: (i8, i8),
    prev_velocity: (i8, i8),
}

impl Cat {
    pub const fn new() -> Self {
        Cat {
            #[rustfmt::skip]
            sprite: [
                0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
                0, 1, 4, 1, 0, 0, 1, 3, 1, 0, 0, 0, 0, 1, 3, 1,
                1, 3, 1, 0, 0, 0, 1, 4, 3, 1, 1, 1, 1, 3, 4, 1,
                1, 3, 1, 0, 0, 0, 1, 3, 4, 4, 4, 4, 4, 4, 3, 1,
                1, 3, 1, 0, 0, 0, 1, 4, 4, 3, 4, 4, 3, 4, 4, 1,
                1, 3, 1, 1, 1, 1, 1, 4, 4, 1, 4, 4, 1, 4, 4, 1,
                1, 3, 4, 3, 3, 4, 1, 4, 4, 4, 4, 4, 4, 4, 4, 1,
                1, 3, 4, 3, 3, 4, 3, 4, 4, 4, 3, 1, 3, 4, 4, 1,
                1, 3, 4, 3, 3, 4, 3, 3, 4, 4, 4, 3, 4, 4, 1, 0,
                1, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 1, 0, 0,
                1, 3, 4, 3, 3, 4, 3, 3, 3, 3, 1, 1, 1, 1, 0, 0,
                1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 3, 1, 0, 0,
                1, 3, 3, 4, 4, 4, 4, 4, 4, 3, 4, 4, 3, 1, 0, 0,
                1, 3, 3, 1, 3, 3, 1, 1, 1, 3, 1, 1, 3, 1, 0, 0,
                1, 4, 1, 1, 4, 1, 0, 0, 1, 4, 1, 1, 4, 1, 0, 0,
                0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
            ],
            state: 0,
            position: (0, 0),
            velocity: (0, 0),
            prev_velocity: (0, 0),
        }
    }

    pub fn update(&mut self, frame_count: u32) {
        if self.velocity == (0, 0) {
            self.state = 0;
        } else {
            let new_position = (
                self.position.0 as i16 + self.velocity.0 as i16,
                self.position.1 as i16 + self.velocity.1 as i16,
            );
            if new_position.0 <= (SCREEN_SIZE - 16) as i16 && new_position.0 >= 0 {
                self.position.0 = new_position.0 as usize;
            }
            if new_position.1 <= (SCREEN_SIZE - 16) as i16 && new_position.1 >= 0 {
                self.position.1 = new_position.1 as usize;
            }
            if frame_count % 15 == 0 {
                if self.state == 1 {
                    self.state = 2;
                } else {
                    self.state = 1;
                }
            }
        }
        match self.state {
            #[rustfmt::skip]
            0 => {
                // Stand
                self.sprite = [
                    0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
                    0, 1, 4, 1, 0, 0, 1, 3, 1, 0, 0, 0, 0, 1, 3, 1,
                    1, 3, 1, 0, 0, 0, 1, 4, 3, 1, 1, 1, 1, 3, 4, 1,
                    1, 3, 1, 0, 0, 0, 1, 3, 4, 4, 4, 4, 4, 4, 3, 1,
                    1, 3, 1, 0, 0, 0, 1, 4, 4, 3, 4, 4, 3, 4, 4, 1,
                    1, 3, 1, 1, 1, 1, 1, 4, 4, 1, 4, 4, 1, 4, 4, 1,
                    1, 3, 4, 3, 3, 4, 1, 4, 4, 4, 4, 4, 4, 4, 4, 1,
                    1, 3, 4, 3, 3, 4, 3, 4, 4, 4, 3, 1, 3, 4, 4, 1,
                    1, 3, 4, 3, 3, 4, 3, 3, 4, 4, 4, 3, 4, 4, 1, 0,
                    1, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 1, 0, 0,
                    1, 3, 4, 3, 3, 4, 3, 3, 3, 3, 1, 1, 1, 1, 0, 0,
                    1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 3, 1, 0, 0,
                    1, 3, 3, 4, 4, 4, 4, 4, 4, 3, 4, 4, 3, 1, 0, 0,
                    1, 3, 3, 1, 3, 3, 1, 1, 1, 3, 1, 1, 3, 1, 0, 0,
                    1, 4, 1, 1, 4, 1, 0, 0, 1, 4, 1, 1, 4, 1, 0, 0,
                    0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
                ];
            }
            #[rustfmt::skip]
            1 => {
                // Movement 1
                self.sprite = [
                    0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
                    1, 4, 1, 0, 0, 0, 1, 3, 1, 0, 0, 0, 0, 1, 3, 1,
                    1, 3, 1, 0, 0, 0, 1, 4, 3, 1, 1, 1, 1, 3, 4, 1,
                    1, 3, 1, 0, 0, 0, 1, 3, 4, 4, 4, 4, 4, 4, 3, 1,
                    1, 3, 1, 0, 0, 0, 1, 4, 4, 3, 4, 4, 3, 4, 4, 1,
                    1, 3, 1, 1, 1, 1, 1, 4, 4, 1, 4, 4, 1, 4, 4, 1,
                    1, 3, 4, 3, 3, 4, 1, 4, 4, 4, 4, 4, 4, 4, 4, 1,
                    1, 3, 4, 3, 3, 4, 3, 4, 4, 4, 3, 1, 3, 4, 4, 1,
                    1, 3, 4, 3, 3, 4, 3, 3, 4, 4, 4, 3, 4, 4, 1, 0,
                    1, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 1, 0, 0,
                    1, 3, 4, 3, 3, 4, 3, 3, 3, 3, 1, 1, 1, 1, 0, 0,
                    1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 1, 0, 0, 0,
                    1, 3, 3, 4, 4, 4, 4, 4, 4, 3, 4, 3, 1, 0, 0, 0,
                    1, 3, 3, 1, 1, 3, 3, 1, 1, 3, 1, 3, 1, 0, 0, 0,
                    1, 4, 1, 0, 1, 4, 1, 0, 1, 4, 1, 4, 1, 0, 0, 0,
                    0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0,
                ];
            }
            #[rustfmt::skip]
            2 => {
                // Movement 2
                self.sprite = [
                    0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0,
                    0, 1, 4, 1, 0, 0, 1, 3, 1, 0, 0, 0, 0, 1, 3, 1,
                    1, 3, 1, 0, 0, 0, 1, 4, 3, 1, 1, 1, 1, 3, 4, 1,
                    1, 3, 1, 0, 0, 0, 1, 3, 4, 4, 4, 4, 4, 4, 3, 1,
                    1, 3, 1, 0, 0, 0, 1, 4, 4, 3, 4, 4, 3, 4, 4, 1,
                    1, 3, 1, 1, 1, 1, 1, 4, 4, 1, 4, 4, 1, 4, 4, 1,
                    1, 3, 4, 3, 3, 4, 1, 4, 4, 4, 4, 4, 4, 4, 4, 1,
                    1, 3, 4, 3, 3, 4, 3, 4, 4, 4, 3, 1, 3, 4, 4, 1,
                    1, 3, 4, 3, 3, 4, 3, 3, 4, 4, 4, 3, 4, 4, 1, 0,
                    1, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 1, 0, 0,
                    1, 3, 4, 3, 3, 4, 3, 3, 3, 3, 1, 1, 1, 1, 0, 0,
                    1, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 3, 1, 0, 0,
                    1, 3, 3, 4, 4, 4, 4, 4, 3, 4, 4, 4, 3, 1, 0, 0,
                    0, 1, 3, 3, 1, 3, 1, 1, 3, 1, 1, 1, 3, 1, 0, 0,
                    0, 1, 4, 1, 4, 1, 0, 1, 4, 1, 0, 1, 4, 1, 0, 0,
                    0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
                ];
            }
            _ => {}
        }
    }

    pub fn draw(&self) {
        let mut flip = false;
        if self.prev_velocity.0 == -1 {
            flip = true;
        }
        draw_sprite(self.position.0, self.position.1, self.sprite, flip);
    }

    pub fn set_velocity(&mut self, velocity: (i8, i8)) {
        if velocity != (0, 0) {
            self.prev_velocity = velocity;
        }
        self.velocity = velocity;
    }
}

static mut GAME: Game = Game::new();

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [0x03324e, 0xc74148, 0xdbb9a0, 0xf5f5f5];
    }
}

#[no_mangle]
fn update() {
    clear_screen(0);
    unsafe {
        GAME.frame_count += 1;
    }
    unsafe {
        GAME.update();
        GAME.input();

        if GAME.frame_count % 3 == 0 {
            GAME.cat.update(GAME.frame_count);
        }
        GAME.cat.draw();
    }
}

fn clear_screen(bg_color: u8) {
    for y in 0..SCREEN_SIZE {
        for x in 0..SCREEN_SIZE {
            draw_pixel(x as usize, y as usize, bg_color);
        }
    }
}

fn draw_sprite(x: usize, y: usize, sprite: [u8; 256], flip: bool) {
    for index in 0..sprite.len() {
        let col = index % 16;
        let row = index / 16;
        let mut sprite_index = index;
        if flip == true {
            sprite_index = (16 * row as i32 + (15 - col as i32).abs()) as usize;
        }
        if sprite[sprite_index] > 0 {
            draw_pixel(x + col, y + row, sprite[sprite_index] - 1);
        }
    }
}

fn draw_pixel(x: usize, y: usize, color: u8) {
    let idx = (y * SCREEN_SIZE as usize + x) >> 2;
    let shift = (x as u8 & 0b11) << 1;
    let mask = 0b11 << shift;
    unsafe {
        let framebuffer = FRAMEBUFFER.as_mut().expect("framebuffer ref");
        framebuffer[idx] = (color << shift) | (framebuffer[idx] & !mask);
    }
}

fn get_gamepad() -> u8 {
    unsafe { *GAMEPAD1 }
}

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo<'_>) -> ! {
    trace("panic error");

    #[cfg(debug_assertions)]
    if let Some(cause) = _panic_info.payload().downcast_ref::<&str>() {
        trace(cause);
    }

    wasm32::unreachable()
}

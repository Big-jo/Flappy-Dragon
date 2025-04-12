use bracket_lib::prelude::*;

struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
}

struct Player {
  x: i32,
  y: i32,
  velcotiy: f32
}

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

impl Player {
  fn new(x: i32, y: i32) -> Self {
    Player{
      x,
      y,
      velcotiy: 0.0
    }
  }

  fn render(&mut self, ctx: &mut BTerm) {
    ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
  }

  fn gravity_and_move(&mut self) {
    if self.velcotiy < 2.0 {
      self.velcotiy += 0.2;
    }

    self.y += self.velcotiy as i32;
    self.x += 1;

    if self.y < 0 {
      self.y = 0;
    }
  }

  fn flap(&mut self) {
    self.velcotiy = -2.0;
  }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(5, 25)
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in this stubb later
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        
        if self.frame_time > FRAME_DURATION {
          self.frame_time = 0.0;

          self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
          self.player.flap();
        }
        
        self.player.render(ctx);
        ctx.print(0, 0, "Press Space To Flap");

        if self.player.y > SCREEN_HEIGHT {
          self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Fuck Yeahhhh Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are Dead Noob");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        ctx.print(5, 5, "Game Over Moda Fuckers");

        if let Some(key) = ctx.key {
          match key {
            VirtualKeyCode::P => self.play(ctx),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
          }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}

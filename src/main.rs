use ggez::conf::WindowMode;
use ggez::glam::Vec2;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use ggez::event::{self, EventHandler};

const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 20;
const DOT_SIZE: usize = 50;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("visual_bfs", "Crocoby")
        .window_mode(WindowMode {
            width: MAP_WIDTH as f32*DOT_SIZE as f32,
            height: MAP_HEIGHT as f32*DOT_SIZE as f32,
            ..Default::default()
        })
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

#[derive(Clone, Copy, Debug)]
struct Point(i32, i32);
#[derive(Clone, Copy, Debug)]
struct Vertex(i32, Point);

struct MyGame {
    map: [[i8; MAP_WIDTH]; MAP_HEIGHT],
    from_point: Point,
    dest_point: Point,
    vertexes: Vec<Vertex>
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut b = MyGame {
            map: [ // Edit your map here (1 = wall, 0 = nothing)
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ],
            from_point: Point(0, 0), // Edit your start position here
            dest_point: Point(19, 19), // Edit your target position here
            vertexes: Vec::new()
        };

        b.map[b.dest_point.1 as usize][b.dest_point.0 as usize] = 0;
        b.map[b.from_point.1 as usize][b.from_point.0 as usize] = 0;

        // Algorithm
        b.vertexes = vec![Vertex(0, b.from_point.clone())];
        let mut queue = vec![Vertex(0, b.from_point.clone())];
        loop {
            let queued = queue.pop();
            if let Some(a) = queued {
                if a.1.0 > 0 && b.map[a.1.1 as usize][(a.1.0-1) as usize]==0 && !b.vertexes.iter().any(|x| {return a.1.0-1 == x.1.0 && a.1.1 == x.1.1}) {
                    let e = Vertex(a.0+1, Point(a.1.0-1, a.1.1));
                    queue.insert(0, e);
                    b.vertexes.push(e);
                    if e.1.0 == b.dest_point.0 && e.1.1 == b.dest_point.1 {
                        break;
                    }
                }
                if a.1.1 > 0 && b.map[(a.1.1-1) as usize][a.1.0 as usize]==0 && !b.vertexes.iter().any(|x| {return a.1.1-1 == x.1.1 && a.1.0 == x.1.0}) {
                    let e = Vertex(a.0+1, Point(a.1.0, a.1.1-1));
                    queue.insert(0, e);
                    b.vertexes.push(e);
                    if e.1.0 == b.dest_point.0 && e.1.1 == b.dest_point.1 {
                        break;
                    }
                }
                if a.1.0+1 < (MAP_WIDTH as i32) && b.map[a.1.1 as usize][(a.1.0+1) as usize]==0 && !b.vertexes.iter().any(|x| {return a.1.1 == x.1.1 && a.1.0+1 == x.1.0}) {
                    let e = Vertex(a.0+1, Point(a.1.0+1, a.1.1));
                    queue.insert(0, e);
                    b.vertexes.push(e);
                    if e.1.0 == b.dest_point.0 && e.1.1 == b.dest_point.1 {
                        break;
                    }
                }
                if a.1.1+1 < (MAP_HEIGHT as i32) && b.map[(a.1.1+1) as usize][a.1.0 as usize]==0 && !b.vertexes.iter().any(|x| {return a.1.0 == x.1.0 && a.1.1+1 == x.1.1}) {
                    let e = Vertex(a.0+1, Point(a.1.0, a.1.1+1));
                    queue.insert(0, e);
                    b.vertexes.push(e);
                    if e.1.0 == b.dest_point.0 && e.1.1 == b.dest_point.1 {
                        break;
                    }
                }
            } else {
                break;
            }

            if queue.len() == 0 {
                break;
            }
        }

        // Returning result
        b
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let (w, h) = ctx.gfx.size();

        // Drawing explored blocks (yellow color)
        for &a in self.vertexes.iter() {
            canvas.draw(&graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(a.1.0 as f32*DOT_SIZE as f32, a.1.1 as f32*DOT_SIZE as f32, DOT_SIZE as f32, DOT_SIZE as f32), Color::YELLOW).unwrap(), DrawParam::new());
            canvas.draw(&graphics::Text::new(a.0.to_string()), DrawParam::new().color(Color::WHITE).dest([a.1.0 as f32*DOT_SIZE as f32, a.1.1 as f32*DOT_SIZE as f32]));
        }

        // Drawing wall blocks (black color)
        for (y, &u) in self.map.iter().enumerate() {
            for (x, &c) in u.iter().enumerate() {
                if c == 1 {
                    canvas.draw(&graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(x as f32*DOT_SIZE as f32, y as f32*DOT_SIZE as f32, DOT_SIZE as f32, DOT_SIZE as f32), Color::BLACK).unwrap(), DrawParam::new());
                }
            }
        }

        // Drawing start position and target position blocks (green and red blocks)
        canvas.draw(&graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(self.from_point.0 as f32*DOT_SIZE as f32, self.from_point.1 as f32*DOT_SIZE as f32, DOT_SIZE as f32, DOT_SIZE as f32), Color::GREEN).unwrap(), DrawParam::new());
        canvas.draw(&graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(self.dest_point.0 as f32*DOT_SIZE as f32, self.dest_point.1 as f32*DOT_SIZE as f32, DOT_SIZE as f32, DOT_SIZE as f32), Color::RED).unwrap(), DrawParam::new());

        let mut prev = self.vertexes.last().unwrap().clone();
        // Drawing red cross when we cant reach to target point
        if prev.1.0 != self.dest_point.0 || prev.1.1 != self.dest_point.1 {
            canvas.draw(
                &graphics::Mesh::new_line(ctx, &[
                                          Vec2::new(prev.1.0 as f32 * DOT_SIZE as f32, prev.1.1 as f32 * DOT_SIZE as f32),
                                          Vec2::new((prev.1.0+1) as f32 * DOT_SIZE as f32, (prev.1.1+1) as f32 * DOT_SIZE as f32)
                ], 3.0, Color::RED).unwrap(),
                DrawParam::new()
            );
            canvas.draw(
                &graphics::Mesh::new_line(ctx, &[
                                          Vec2::new(prev.1.0 as f32 * DOT_SIZE as f32, (prev.1.1+1) as f32 * DOT_SIZE as f32),
                                          Vec2::new((prev.1.0+1) as f32 * DOT_SIZE as f32, prev.1.1 as f32 * DOT_SIZE as f32)
                ], 3.0, Color::RED).unwrap(),
                DrawParam::new()
            );
        }

        // Drawing path lines (red color)
        for &a in self.vertexes.iter().rev() {
            if prev.0-1 == a.0 && (prev.1.0-a.1.0).abs() <= 1 && (prev.1.1-a.1.1).abs() <= 1 {
                canvas.draw(
                    &graphics::Mesh::new_line(ctx, &[
                                              Vec2::new(prev.1.0 as f32 * DOT_SIZE as f32 + (DOT_SIZE as f32 * 0.5), prev.1.1 as f32 * DOT_SIZE as f32 + (DOT_SIZE as f32 * 0.5)),
                                              Vec2::new(a.1.0 as f32 * DOT_SIZE as f32 + (DOT_SIZE as f32 * 0.5), a.1.1 as f32 * DOT_SIZE as f32 + (DOT_SIZE as f32 * 0.5))
                    ], 3.0, Color::RED).unwrap(),
                    DrawParam::new()
                );
                prev = a.clone();
            }
        }

        // Drawing grid from horizontal and vertical lines (black color)
        for x in (0..=(w as usize)).step_by(DOT_SIZE) {
            canvas.draw(&graphics::Mesh::new_line(ctx, &[
                                              Vec2::new(x as f32, 0.0),
                                              Vec2::new(x as f32, h)
            ], 1.0, Color::BLACK).unwrap(), DrawParam::new());
        }

        for y in (0..=(h as usize)).step_by(DOT_SIZE) {
            canvas.draw(&graphics::Mesh::new_line(ctx, &[
                                              Vec2::new(0.0, y as f32),
                                              Vec2::new(w, y as f32)
            ], 1.0, Color::BLACK).unwrap(), DrawParam::new());
        }


        // Showing changes
        canvas.finish(ctx)
    }
}

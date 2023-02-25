use std::env;

use colored::Colorize;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use std::time::Duration;

fn init_canvas(context: &Sdl) -> Canvas<sdl2::video::Window> {
    // SDL has different "subsystems" that you can initialize. Here we initialize the video subsystem.
    // See the docs for more details: https://www.libsdl.org/release/SDL-1.2.15/docs/html/guidebasicsinit.html
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem
        .window("HackHer Workshop 2023", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    canvas
}

fn run_demo(sdl_context: &Sdl) {
    let mut canvas = init_canvas(&sdl_context);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn run_square(sdl_context: &Sdl) {
    let mut canvas = init_canvas(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut rect = Rect::new(50, 50, 100, 100);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.clear();
        
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        match canvas.fill_rect(rect) {
            Ok(_) => {},
            Err(s) => {
                eprintln!("There was an error drawing the rectangle: {}", s);
                return;
            }
        };
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {rect.set_x(rect.x + 10)},
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {rect.set_x(rect.x - 10)},
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {rect.set_y(rect.y - 10)},
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {rect.set_y(rect.y + 10)},
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Entity {
    val: i32
}

fn move_and_return(e: Entity) -> Entity {
    println!("Thanks for the variable with value '{:?}'. I'll give it back now.", e);
    return e
}

#[allow(dead_code)]
fn move_and_steal(e: Entity) {
    println!("I've stolen your variable with value '{:?}' and you can't have it back!", e);
}

#[allow(dead_code)]
fn borrow(e: &Entity) {
    println!("I'm just borrowing your variable with value '{:?}', but I never owned it!", e);
}

fn run_borrow() {
    let mut e = Entity { val: 5 };

    e = move_and_return(e);
    // move_and_steal(e);
    // borrow(&e);

    println!("Printing the Entity:\n{:?}", e);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    // we need the ampersand (&) before args to tell the compiler we only want to pass dbg! a temporary reference to args, not "move" the value.
    dbg!(&args);

    let accepted_args = vec!["demo", "square", "borrow"];

    // some funky command-line parsing showing off some basic rust string/vector manipulation features
    if args.len() > 2 {
        println!("Usage: {} -- <run-mode>", args[0]);
        return;
    } else if args.len() == 2 && !accepted_args.contains(&args[1].as_str()) {
        eprint!(
            "{} {} {} {} ",
            "Error:".bold().red(),
            "argument".red(),
            args[1].red(),
            "not in accepted arg list:".red()
        );

        // for anyone who has taken 220: this map expression might interest you!
        let mut accepted_args_colored = accepted_args
            .into_iter()
            .map(|s| s.to_string().green())
            .peekable();

        // pretty-printing the list of accepted args colored green
        while let Some(arg) = accepted_args_colored.next() {
            eprint!("{}{}", arg, if accepted_args_colored.peek().is_none() { "" } else { ", " });
        }
        eprintln!();

        return;
    }

    // initialize the main OpenGL context for SDL2
    let sdl_context = match sdl2::init() {
        Ok(context) => context,
        Err(err) => {
            println!("Something went terribly wrong while initializing SDL!: {}", err);
            return;
        }
    };

    match (args.len(), if args.len() == 2 { args[1].as_str() } else { "" }) {
        (1, _) => run_demo(&sdl_context),
        (2, "demo") => run_demo(&sdl_context),
        (2, "square") => run_square(&sdl_context),
        (2, "borrow") => run_borrow(),
        _ => println!("No run mode specified, running demo"),
    }
}

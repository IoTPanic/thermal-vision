// Used the getting started piston code as a base https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    values: [[f32; 8]; 8],
    last_updated: f64,
    line: i32,
    max_value: f32,
}

impl App {
    fn render(&mut self, args: &RenderArgs, values: [[f32; 8]; 8]) {
        use graphics::*;
        let M = self.max_value;
        self.gl.draw(args.viewport(), |c, gl| {
            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            // Clear the screen.
            clear(GREEN, gl);

            let mut x = 0.0;
            let mut y = 0.0;
            for value_y in 0..8 {
                x = 0.0;
                for value_x in 0..8 {
                    //let square = rectangle::square(0.0, 0.0, args.window_size[0] / 8.0);
                    let trans = c.transform.trans(
                        (args.window_size[0] / 8.0) * x,
                        (args.window_size[1] / 8.0) * y,
                    );
                    
                    let mut color = colorFromValue(values[value_y][value_x], 0.0, M);

                    // color[0] = 1.0;
                    // color[1] = 0.0;
                    // color[2] = 0.0;

                    println!("Color: {:#?}", color);
                    let s = [0.0, 0.0, args.window_size[0] / 8.0, args.window_size[1] / 8.0];
                    rectangle(color, s, trans, gl);
                    x += 1.0;
                }
                y += 1.0;
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, filename: &String, values: &mut [[f32; 8]; 8]) {
        self.last_updated += args.dt;
        
        if self.last_updated >= 0.2{
            self.last_updated = 0.0;
            let (cv, r) = read(filename, self.line);
            if !r{
                println!("Ran {} frames", self.line);
                std::process::exit(0);
                // return;
            }
            self.line += 1;
            
            for y in 0..8{
                for x in 0..8{
                    //cv[y][x] = map(cv[y][x], range_min, range_max, 0.0, 1.0);
                    //println!("{}", values[y][x]);
                    values[y][x] = cv[y][x];
                    //println!("{}", values[y][x]);
                }
            }
        }
    }
}

fn colorFromValue(val: f32, min: f32, max: f32) -> [f32;4] {
    let mut result = [0.0;4];
    result[3] = 1.0;
    println!("{:#?}",max);
    // result[0] = ((val-min) / (max-min) * 250.0 + (val-min)/(max-min) * 300.0) / 100.0;
    // result[1] = ((val-min) / (max-min) * 150.0 + (val-min)/(max-min) * 200.0) / 100.0;
    // result[2] = ((val-min) / (max-min) * 0.0 + (val-min)/(max-min) * 50.0) / 100.0;

    result[0] = map(val, 0.0, max, 0.0, 1.0);
    result[1] = map(val, 0.0, max, 0.0, 1.0);
    result[2] = map(val, 0.0, max, 0.0, 1.0);

    //println!("{:#?}", result);
    return result;
} 

fn map(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

fn read(filename: &String, line_num: i32) -> ([[f32; 8]; 8], bool) {
    let mut result = [[0.0; 8]; 8];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        if i as i32 != line_num {
            continue;
        }

        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  // println!("{}. {}", index + 1, line);

        let mut index = 0;
        for s in line.split(",") {
            if s == "" {
                continue;
            }
            let f = s.parse::<f32>();
            match f {
                Ok(f) => {
                    result[index / 8][index % 8] = f;
                }
                Err(e) => {
                    println!("Could not parse value!");
                    continue;
                }
            }
            index += 1;
        }
        return (result, true);
    }
    return (result, false);
}

fn getMaxValue(filename: &String) -> f32 {
    let mut r = 0.0;

    let mut line = 0;
    while line<99999 { 
        let (values, succ) = read(filename, line);
        if !succ {
            break;
        }
        for y in 0..8 {
            for x in 0..8 {
                if values[y][x] >  r {
                    r = values[y][x];
                    if r > 1200.0 {
                        r = 1200.0;
                    }
                }
            }
        }
        line += 1;
    }

    return r;
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        println!("Please include a filename!");
        std::process::exit(1);
    }

    let filename = &args[1];

    let mut window: Window = WindowSettings::new("Thermal Playback", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut app = App {
        gl: GlGraphics::new(opengl),
        values: [[0.0; 8]; 8],
        last_updated: 0.0,
        line: 0,
        max_value: getMaxValue(filename),
    };

    let mut events = Events::new(EventSettings::new());

    let mut vals = [[0.0; 8]; 8];

    if app.max_value==0.0 {
        println!("Error could not range document");
        return;
    }

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, vals);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, filename, &mut vals);
        }
    }
    println!("max {}", app.max_value);
    println!("Ran {} frames", app.line);
}

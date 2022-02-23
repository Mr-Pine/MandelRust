use ferris_says::say;
use std::io::{stdout, BufWriter};

const MAX_ITERATIONS: i32 = 100;

const UPPER_MANDELBROT_X: f64 = 0.47f64;
const LOWER_MANDELBROT_X: f64 = -2f64;
const RANGE_SIZE_X: f64 = UPPER_MANDELBROT_X - LOWER_MANDELBROT_X;

const UPPER_MANDELBROT_Y: f64 = 1.12f64;
const LOWER_MANDELBROT_Y: f64 = -1.12f64;
const RANGE_SIZE_Y: f64 = UPPER_MANDELBROT_Y - LOWER_MANDELBROT_Y;

const CHARACTER_SIZE_COMPENSATION: f64 = 2.5;

const LUMINANCE_SEQUENCE: &str = ".,-~:;=!*#$@";

fn main() {
    let message = String::from("Printing the mandelbrot set");
    say(message.as_bytes(), message.len(), &mut BufWriter::new(stdout().lock())).unwrap();
    let mut screen = Screen { width: 300, height: (300f64/CHARACTER_SIZE_COMPENSATION) as usize, pixels: vec![], scale: 1f64, iteration_count: vec![], total_iterations: 0 };
    screen.initialize();

    //dbg!(screen.pixels);
    screen.calculate_set();
    screen.print()
}

struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<Vec<MandelbrotResult>>,
    scale: f64,
    iteration_count: Vec<isize>,
    total_iterations: isize
}

impl Screen {
    fn initialize(&mut self) {

        self.pixels = vec![vec![MandelbrotResult::Uninitialized; self.width]; self.height];

        self.scale = self.get_scale_factor()
    }

    fn print(&mut self) {
        let mut buffer = String::with_capacity(self.width * self.height);

        self.calculate_iterations();

        dbg!(self.total_iterations);

        for (y, y_vec) in self.pixels.iter().enumerate() {
            for (x, _x_val) in y_vec.iter().enumerate() {
                let pixel = match self.pixels[y][x] {
                    MandelbrotResult::Escapes(_repetitions) => {
                        //let index = ((f64::log(_repetitions as f64, MAX_ITERATIONS as f64)) as f64 * (LUMINANCE_SEQUENCE.len() - 1) as f64) as usize;
                        let i = match self.pixels[y][x] {
                            MandelbrotResult::Escapes(i) => i as usize,
                            MandelbrotResult::Bounded => 0,
                            MandelbrotResult::Uninitialized => 0
                        };
                        let index = (self.iteration_count[..i].iter().sum::<isize>() as f64 / self.total_iterations as f64 * (LUMINANCE_SEQUENCE.len() - 1) as f64) as usize;
                        &LUMINANCE_SEQUENCE[index..index+1]
                    }
                    MandelbrotResult::Bounded => " ",
                    MandelbrotResult::Uninitialized => ""
                };
                buffer += &*pixel;
            }
            buffer += "\n";
        }

        print!("{}", buffer)
    }


    fn get_scale_factor(&self) -> f64 {
        let a = (UPPER_MANDELBROT_X - LOWER_MANDELBROT_X) / self.width as f64;
        let b = (UPPER_MANDELBROT_Y - LOWER_MANDELBROT_Y) / (self.height as f64 * CHARACTER_SIZE_COMPENSATION);
        dbg!(a);
        dbg!(b);
        dbg!(if a > b {
            a
        } else {
            b
        })
    }

    fn calculate_set(&mut self){
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_value = get_mandelbrot_value(ComplexNumber {
                    real: self.scale * (x as f64 + LOWER_MANDELBROT_X * self.width as f64 / RANGE_SIZE_X),
                    imaginary: CHARACTER_SIZE_COMPENSATION * self.scale * (y as f64 + LOWER_MANDELBROT_Y * self.height as f64 / RANGE_SIZE_Y),
                });

                self.pixels[y][x] = pixel_value;
            }
        }
    }

    fn calculate_iterations(&mut self) {
        self.iteration_count.resize(MAX_ITERATIONS as usize +1, 0);
        for y in 0..self.height {
            for x in 0..self.width {
                self.iteration_count[
                    match self.pixels[y][x] {
                        MandelbrotResult::Escapes(i) => i as usize,
                        MandelbrotResult::Bounded => 0,
                        MandelbrotResult::Uninitialized => 0
                    }
                ] += 1;
            }
        }

        self.total_iterations = self.iteration_count.iter().sum()
    }
}

#[derive(Debug, Copy, Clone)]
enum MandelbrotResult {
    Escapes(i32),
    Bounded,
    Uninitialized
}

#[derive(Debug)]
struct ComplexNumber {
    real: f64,
    imaginary: f64,
}

impl ComplexNumber {
    fn clone(&self) -> ComplexNumber {
        ComplexNumber { real: self.real, imaginary: self.imaginary }
    }
}

fn get_mandelbrot_value(c: ComplexNumber) -> MandelbrotResult {
    let mut z = c.clone();
    for i in 0..MAX_ITERATIONS {
        z = ComplexNumber {
            real: (z.real * z.real) - (z.imaginary * z.imaginary) + c.real,
            imaginary: (2f64 * z.real * z.imaginary) + c.imaginary,
        };
        if z.imaginary > 2f64 || z.real > 2f64 {
            return MandelbrotResult::Escapes(i);
        }
    }
    MandelbrotResult::Bounded
}
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::{thread, time};
use std::convert::TryInto;

struct sdl_man
{
    context: sdl2::Sdl,
    vid: sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas,
    wwidth: u32,
    wheight: u32
}

pub struct Plotter<'a>
{
    xs: &'a Vec::<f32>,
    ys: &'a Vec::<f32>,
    xstart: f32,
    xend: f32,
    ystart: f32,
    yend: f32,
    video: sdl_man
}

impl sdl_man
{
    pub fn new(width: u32, height: u32, title: &str) -> sdl_man {
        let ctx = sdl2::init().unwrap();
        let vidsub = ctx.video().unwrap();
        let cnv = vidsub.window(title, width, height)
            .position_centered()
            .build()
            .unwrap()
            .into_canvas()
            .build()
            .unwrap();
        sdl_man { context: ctx, vid: vidsub, canvas: cnv, wwidth: width, wheight: height }
    }

    pub fn clear_canvas(&mut self, col: &Color) {
        let currcol = self.canvas.draw_color();
        self.canvas.set_draw_color(*col);
        self.canvas.clear();
        self.canvas.set_draw_color(currcol);
    }

    pub fn present_canvas(&mut self) {
        self.canvas.present();
    }

    pub fn hide_canvas(&mut self) {
        self.canvas.window_mut().hide();
    }

    pub fn show_canvas(&mut self) {
        self.canvas.window_mut().show();
    }

    pub fn set_draw_color(&mut self, col: &Color) {
        self.canvas.set_draw_color(*col);
    }

    pub fn draw_line_raw(&mut self, c1: (u32, u32), c2: (u32, u32)) {
        let pt1 = Point::new(c1.0 as i32, c1.1 as i32);
        let pt2 = Point::new(c2.0 as i32, c2.1 as i32);
        self.canvas.draw_line(pt1, pt2);
    }

    pub fn draw_square(&mut self, loc: (u32, u32), size: u32) {
        let rect = Rect::from_center(Point::new(loc.0.try_into().unwrap(), loc.1.try_into().unwrap()), size, size);
        self.canvas.draw_rect(rect);
    }

    pub fn get_width(&self) -> u32 {
        self.wwidth
    }

    pub fn get_height(&self) -> u32{
        self.wheight
    }
}

impl<'a> Plotter<'a>
{
    /*
     * throw error if xs.len() != ys.len() ?
     */
    pub fn new(xs: &'a Vec::<f32>, ys: &'a Vec::<f32>) -> Plotter<'a>
    {
        let width = 800;
        let height = 600;
        let mut xstart = f32::MAX;
        let mut xend = f32::MIN;
        let mut ystart = f32::MAX;
        let mut yend = f32::MIN;

        let zipiter = xs.iter().zip(ys.iter());

        for (x, y) in zipiter {
            if *x < xstart {
                xstart = *x;
            }
            if *x > xend {
                xend = *x;
            }
            if *y < ystart {
                ystart = *y;
            }
            if *y > yend {
                yend = *y;
            }
        }

        let xrange = xend - xstart;
        let yrange = yend - ystart;

        xend += xrange * 0.05f32;
        xstart -= xrange * 0.05f32;
        yend += yrange * 0.05f32;
        ystart -= yrange * 0.05f32;

        Plotter {xs: xs, 
            ys: ys, 
            video: sdl_man::new(width, height, "Window test"), 
            xstart: xstart,
            xend: xend,
            ystart: ystart,
            yend: yend}
    }

    pub fn show_window_delay(&mut self, ms: u32) {
        self.video.show_canvas();

        self.video.clear_canvas(&Color::RGB(255, 0, 0));
        self.video.present_canvas();

        self.video.set_draw_color(&Color::RGB(0, 0, 0));
        self.video.draw_line_raw((100, 100), (500, 300));
        self.video.present_canvas();

        thread::sleep(time::Duration::from_millis(ms.into()));

        self.video.hide_canvas();
    }

    fn get_f32_vidwidth(&self) -> f32 {
        self.video.get_width() as f32
    }

    fn get_f32_vidheight(&self) -> f32 {
        self.video.get_height() as f32
    }

    pub fn show_points(&mut self, ms: u32, size: u32) {
        self.video.show_canvas();

        self.video.clear_canvas(&Color::RGB(255, 0, 0));
        self.video.present_canvas();

        self.video.set_draw_color(&Color::RGB(0, 0, 0));

        for i in 0..self.xs.len() {
            let xui = (((self.xs[i] - self.xstart) / (self.xend - self.xstart)) * self.get_f32_vidwidth()) as u32;
            let yui = (((self.ys[i] - self.ystart) / (self.yend - self.ystart)) * self.get_f32_vidheight()) as u32;
            self.video.draw_square((xui, yui), size);
            println!("{}", format!("drawing square at: {}, {}", xui, yui));
            //println!("Actual coordinates at {}, {}", self.xs[i], self.ys[i]);
        }

        self.video.present_canvas();

        thread::sleep(time::Duration::from_millis(ms.into()));

        self.video.hide_canvas();

    }


}

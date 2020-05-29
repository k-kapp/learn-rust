extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Point};
use std::{thread, time};

struct sdl_man
{
    context: sdl2::Sdl,
    vid: sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas
}

pub struct Plotter<'a>
{
    xs: &'a Vec::<f32>,
    ys: &'a Vec::<f32>,
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
        sdl_man { context: ctx, vid: vidsub, canvas: cnv }
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
}

impl<'a> Plotter<'a>
{
    pub fn new(xs: &'a Vec::<f32>, ys: &'a Vec::<f32>) -> Plotter<'a>
    {
        let width = 800;
        let height = 600;
        Plotter {xs: xs, ys: ys, video: sdl_man::new(width, height, "Window test")}
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

}

mod model;
use criterion_plot::prelude::*;

fn main() {
    println!("Hello, world!");
    let m = model::model::new(1.0, 1.0, 1.0, 1.0, 1.0);
    let tup = m.simulate(1000);
    let mut xs = tup.0;
    let mut ys = tup.1;
    let mut yspredict = m.predict_vals(&xs);

    for i in 0..1000
    {
        println!("{}, {}", xs[i], ys[i]);
    }

    Figure::new().plot(
        Points { x: &xs, y: &ys },
        |lp| {
            lp.set(Color::Red)
        }).
        plot(
        Lines { x: &xs, y: &yspredict},
        |lp| {
            lp.set(Color::Blue)
        })
        .draw().unwrap();

    /*
    Figure::new()
        .configure(Key, |k| {
            k.set(Boxed::Yes)
             .set(Position::Inside(Vertical::Top, Horizontal::Left))
        })
        .plot(LinesPoints {
                  x: &xs,
                  y: &ys,
              },
              |lp| {
                  lp.set(Color::DarkViolet)
                    .set(Label("sin(x)"))
                    .set(LineType::Dash)
                    .set(PointSize(1.5))
                    .set(PointType::Circle)
              })
        .plot(Steps {
                  x: &xs,
                  y: &ys,
              },
              |s| {
                  s.set(Color::Rgb(0, 158, 115))
                   .set(Label("atan(x)"))
                   .set(LineWidth(2.))
              })
        .plot(Impulses {
                  x: &xs,
                  y: &ys,
              },
              |i| {
                  i.set(Color::Rgb(86, 180, 233))
                   .set(Label("cos(atan(x))"))
              })
        .draw().unwrap();  // (rest of the chain has been omitted)
    */


}

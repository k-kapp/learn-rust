mod model;
mod csv_reader;
mod dataframe;
mod plotter;
use criterion_plot::prelude::*;

fn plotdata(xs: &Vec::<f32>, ys: &Vec::<f32>, yspredict: &Vec::<f32>)
{
    Figure::new().plot(
        Points { x: xs, y: ys },
        |lp| {
            lp.set(Color::Red)
        }).
        plot(
        Lines { x: xs, y: yspredict},
        |lp| {
            lp.set(Color::Blue)
        })
        .draw().unwrap();
}

fn main() {
    let m = model::model::new(1.0, 1.0, 1.0, 1.0, 1.0);
    let tup = m.simulate(1000, 0.0, 100.0).unwrap();
    let mut xs = tup.0;
    let mut ys = tup.1;
    let mut yspredict = m.predict_vals(&xs);

    for i in 0..1000
    {
        println!("{}, {}", xs[i], ys[i]);
    }

    let df = csv_reader::read_csv("/home/konrad/rust/bootstrap/data1.csv");

    println!("Dataframe:");
    println!("{}", df);

    let dfx = df.getcoli_float(0).unwrap();
    let dfy = df.getcoli_float(1).unwrap();

    let model = model::model_est::new(&dfx, &dfy);
    let dpredicts = model.get_predictions();

    //plotdata(&dfx, &dfy, &dpredicts);

    model.report_basic();

    let xmin = dfx.clone().iter().fold(std::f32::MAX, |a, e| if e < &a { *e } else { a } );
    let xmax = dfx.clone().iter().fold(-std::f32::MAX, |a, e| if e > &a { *e } else { a } );
    let nelements = dfx.len();

    println!("Minimum, maximum x: {}, {}", xmin, xmax);
    println!("Number of elements: {}", nelements);

    let xys_sim = model.simulate_parametric((nelements * 1) as u64, xmin, xmax).unwrap();

    let model_simulated = model::model_est::new(&xys_sim.0, &xys_sim.1);
    let simpredicts = model_simulated.get_predictions();

    model_simulated.report_basic();

    plotdata(&xys_sim.0, &xys_sim.1, &simpredicts);

    let xys_sim_nonp = model.simulate_nonparam_pairs(nelements as u64).unwrap();

    let model_nonparam = model::model_est::new(&xys_sim_nonp.0, &xys_sim_nonp.1);

    let mut pl = plotter::Plotter::new(&xys_sim_nonp.0, &xys_sim_nonp.1);

    pl.show_window_delay(1000);

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

extern crate rand;
use rand::Rng;
use rand::distributions::{Normal, Distribution};
use itertools_num::linspace;

pub struct model
{
    beta0: f32,
    beta1: f32,
    sigma: f32,
    gamma: f32,
    lambda: f32,
    normal: rand::distributions::Normal
}

impl model
{
    pub fn new(beta0: f32, beta1: f32, sigma: f32, gamma: f32, lambda: f32) -> model
    {
        model {beta0: beta0, 
            beta1: beta1, 
            sigma: sigma,
            gamma: gamma,
            lambda: lambda,
            normal: rand::distributions::Normal::new(0.0f64, 1.0f64)}
    }

    pub fn evaluate(&self, rng: &mut rand::rngs::ThreadRng) -> (f32, f32)
    {
        let x = rng.gen::<f32>() * 10.0f32;
        let e = self.normal.sample(rng) as f32;
        (x, self.beta0 + self.beta1 * x + e)
    }

    pub fn simulate(&self, n: u64) -> (Vec::<f32>, Vec::<f32>)
    {
        let mut vec = Vec::<(f32, f32)>::new();
        let mut rng = rand::thread_rng();

        for i in 0..n
        {
            vec.push(self.evaluate(&mut rng))
        }

        //let mut xy: Vec<(f32, f32)> = xs.iter().map(|&x| x).zip(vec.iter().map(|&y| y)).collect();        // when I still created this with two separate vecs, xs and ys

        vec.sort_by(|xy1, xy2| xy1.0.partial_cmp(&xy2.0).unwrap());
        vec.iter().map(|&(a, b)| (a, b)).unzip()
        //vec.iter().unzip()
    }

    pub fn predict_n(&self, n: u64, start: f32, stop: f32) -> (Vec::<f32>, Vec::<f32>)
    {
        let mut ys = Vec::<f32>::new();
        //let mut xs = Vec::<f32>::new();
        let mut xs = linspace::<f32>(start, stop, n as usize).collect::<Vec<_>>();
        let mut rng = rand::thread_rng();

        for x in &xs
        {
            let temp = self.predict(*x);
            ys.push(temp.1);
        }
        (xs, ys)
    }

    pub fn predict_vals(&self, xs: &Vec::<f32>) -> Vec::<f32>
    {
        let mut ys = Vec::<f32>::new();
        let mut rng = rand::thread_rng();

        for x in xs
        {
            let temp = self.predict(*x);
            ys.push(temp.1);
        }
        ys
    }

    pub fn predict(&self, x: f32) -> (f32, f32)
    {
        (x, self.beta0 + self.beta1 * x)
    }

    pub fn estimate_simple(&self, xdata: &Vec::<f32>, ydata: &Vec::<f32>) -> (f32, f32)
    {
        let mut xmean: f32 = 0.0f32;
        let mut ymean: f32 = 0.0f32;

        let xyiter = xdata.iter().zip(ydata.iter());
        let xyiter2 = xyiter.clone();
        
        for (x, y) in xyiter
        {
            xmean += x;
            ymean += y;
        }

        xmean /= (xdata.len() as u32 as f32); 
        ymean /= (xdata.len() as u32 as f32);

        let mut b0_est: f32 = 0.0f32;
        let mut b1_est: f32 = 0.0f32;
        
        for (x, y) in xyiter2
        {
            b1_est += (x - xmean) * (y - ymean) / ((x - xmean) * (x - xmean));
        }
        b0_est = ymean - b1_est * xmean;

        (b0_est, b1_est)
    }
}

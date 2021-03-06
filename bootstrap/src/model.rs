extern crate rand;
use rand::Rng;
use rand::distributions::{Normal, Distribution};
use itertools_num::linspace;


fn derive_sim_models_parametric(modl: &model_est, n: u64) -> Vec::<model_est> {
    let mut simmods = Vec::<model_est>::new();
    let nelements = modl.get_nelements();
    let xmin = 0.0f32;
    let xmax = 0.0f32;

    for i in 0..n {
        let xys_sim = modl.simulate_parametric(nelements, xmin, xmax).unwrap();
        simmods.push(model_est::new(&xys_sim.0, &xys_sim.1));
    }
    simmods
}

fn derive_sim_models_nonparametric_pairs(modl: &model_est, n: u64) -> Vec::<model_est> {
    let mut simmods = Vec::<model_est>::new();
    let nelements = modl.get_nelements();

    for i in 0..n {
        let xys_sim = modl.simulate_nonparam_pairs(nelements).unwrap();
        simmods.push(model_est::new(&xys_sim.0, &xys_sim.1));
    }
    simmods
}

pub struct model
{
    beta0: f32,
    beta1: f32,
    sigma: f32,
    gamma: f32,
    lambda: f32,
    normal: rand::distributions::Normal
}

pub struct model_est
{
    params: model,
    xdata: Vec::<f32>,
    ydata: Vec::<f32>,
    ypred: Vec::<f32>,
    resids: Vec::<f32>,
    resid_se: f32,
    beta0_se: f32,
    beta1_se: f32,
    ssex: f32,
    ssey: f32,
    ssexy: f32,
    xmean: f32,
    ymean: f32
}

impl model_est
{
    pub fn new(xdata: &Vec::<f32>, ydata: &Vec::<f32>) -> model_est {
        let mut ret_model = model_est {
            params: model::new(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32),
            xdata: xdata.to_vec(),
            ydata: ydata.to_vec(),
            ypred: Vec::<f32>::new(),
            resids: Vec::<f32>::new(),
            resid_se: 0.0f32,
            beta0_se: 0.0f32,
            beta1_se: 0.0f32,
            ssex:     0.0f32,
            ssey:     0.0f32,
            ssexy:    0.0f32,
            xmean:    0.0f32,
            ymean:    0.0f32
        };

        ret_model.estimate_all();

        ret_model
    }

    fn estimate(&mut self) -> Result<(), String> {
        if self.xdata.len() != self.ydata.len()
        {
            return Err("X data must be of same length as Y data in estimate_simple".to_string());
        }
        if self.xdata.len() < 3
        {
            return Err("Length of data in estimate_simple must be more than 3".to_string());
        }

        self.xmean = 0.0f32;
        self.ymean = 0.0f32;

        let xyiter = self.xdata.iter().zip(self.ydata.iter());
        let xyiter2 = xyiter.clone();
        
        for (x, y) in xyiter
        {
            self.xmean += x;
            self.ymean += y;
        }

        self.xmean /= (self.xdata.len() as u32 as f32); 
        self.ymean /= (self.xdata.len() as u32 as f32);

        //let mut b0_est: f32 = 0.0f32;
        //let mut b1_est: f32 = 0.0f32;

        self.ssex = 0.0f32;
        self.ssey = 0.0f32;
        self.ssexy = 0.0f32;
        
        for (x, y) in xyiter2
        {
            self.ssexy += (x - self.xmean) * (y - self.ymean);
            //b1_est += (x - xmean) * (y - ymean) / ((x - xmean) * (x - xmean));
            self.ssex += (x - self.xmean) * (x - self.xmean);
            self.ssey += (y - self.ymean) * (y - self.ymean);
        }
        let b1_est = self.ssexy / self.ssex;
        let b0_est = self.ymean - b1_est * self.xmean;

        self.params = model::new(b0_est, b1_est, 0.0f32, 0.0f32, 0.0f32);

        //self.beta1_se = ((self.ssey / ((self.xdata.len() - 2) as f32)) / self.ssex).sqrt();

        Ok(())
    }

    pub fn get_nelements(&self) -> u64 {
        self.xdata.len() as u64
    }

    pub fn get_xextr(&self, min: &mut f32, max: &mut f32) {
        *min = self.xdata.clone().iter().fold(std::f32::MAX, |a, e| if e < &a { *e } else { a } );
        *max = self.xdata.clone().iter().fold(-std::f32::MAX, |a, e| if e > &a { *e } else { a } );
    }

    pub fn estimate_all(&mut self) {
        self.estimate();
        self.ypred = self.params.predict_vals(&self.xdata);
        self.compute_resids();
        self.beta0_se = (self.resid_se * self.resid_se * (1f32 / (self.xdata.len() as f32) + self.xmean * self.xmean / self.ssex)).sqrt();
        self.beta1_se = (self.resid_se * self.resid_se / self.ssex).sqrt();

        let mut beta0 = 0.0f32;
        let mut beta1 = 0.0f32;
        self.params.get_betas(&mut beta0, &mut beta1);
        self.params = model::new(beta0, beta1, self.resid_se, 0.0f32, 0.0f32);       // update with new params, containing additional residual SE
    }

    fn compute_resids(&mut self) {
        self.resids.clear();
        let mut sqsum = 0.0f32;
        for (yo, yp) in self.ydata.iter().zip(self.ypred.iter()) {
            self.resids.push(yp - yo);
            sqsum += (yp - yo) * (yp - yo);
        }

        self.resid_se = (sqsum / ((self.ydata.len() as f32 - 1.0f32) as f32)).sqrt();
    }

    pub fn get_predictions(&self) -> Vec::<f32> {
        self.params.predict_vals(&self.xdata)
    }

    pub fn report_basic(&self) {
        let mut beta0: f32 = 0.0f32;
        let mut beta1: f32 = 0.0f32;
        self.params.get_betas(&mut beta0, &mut beta1);
        println!("beta0: {}, beta1: {}", beta0, beta1);
        println!("beta0_se: {}, beta1_se: {}", self.beta0_se, self.beta1_se);
    }

    pub fn simulate_parametric(&self, n: u64, start: f32, stop: f32) -> Result<(Vec::<f32>, Vec::<f32>), String> {
        self.params.simulate(n, start, stop)
    }

    pub fn simulate_nonparam_pairs(&self, n: u64) -> Result<(Vec::<f32>, Vec::<f32>), String> {
        let mut rng = rand::thread_rng();
        let uniform = rand::distributions::Uniform::new(0 as u32, self.xdata.len() as u32);
        let mut sxs = Vec::<f32>::new();
        let mut sys = Vec::<f32>::new();

        for i in 0..n {
            let ridx = uniform.sample(&mut rng);
            sxs.push(self.xdata[ridx as usize]);
            sys.push(self.ydata[ridx as usize]);
        }

        Ok((sxs, sys))
    }
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

    fn evaluate(&self, rng: &mut rand::rngs::ThreadRng, start: f32, stop: f32) -> (f32, f32)
    {
        let range = stop - start;
        let x = rng.gen::<f32>() * range + start;
        let e = (self.normal.sample(rng) as f32) * self.sigma;
        (x, self.beta0 + self.beta1 * x + e)
    }

    pub fn simulate(&self, n: u64, start: f32, stop: f32) -> Result<(Vec::<f32>, Vec::<f32>), String>
    {
        let mut vec = Vec::<(f32, f32)>::new();
        let mut rng = rand::thread_rng();

        if stop <= start
        {
            return Err("Invalid range given for start, stop in model::simulate".to_string());
        }

        for i in 0..n
        {
            vec.push(self.evaluate(&mut rng, start, stop))
        }

        //let mut xy: Vec<(f32, f32)> = xs.iter().map(|&x| x).zip(vec.iter().map(|&y| y)).collect();        // when I still created this with two separate vecs, xs and ys

        vec.sort_by(|xy1, xy2| xy1.0.partial_cmp(&xy2.0).unwrap());
        Ok(vec.iter().map(|&(a, b)| (a, b)).unzip())
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

    pub fn get_betas(&self, beta0: &mut f32, beta1: &mut f32) {
        *beta0 = self.beta0;
        *beta1 = self.beta1;
    }

}

use ndarray;

pub struct Particle {
    pub position: ndarray::Array2<f64>,
    pub velocity: ndarray::Array2<f64>,
    pub acceleration: f64
}
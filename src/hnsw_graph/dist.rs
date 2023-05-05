use num_traits::float::*;

#[allow(unused)]
enum DistKind {
    DistDot(String), //TODO: why String?
    DistCosine(String),
    /// Distance defined by a closure
    DistFn,
    /// Distance defined by a fn Rust pointer
    DistPtr,
}

/// This is the basic Trait describing a distance. The structure Hnsw can be instantiated by anything
/// satisfying this Trait.
pub trait Distance<T: Send + Sync> {
    fn eval(&self, va: &[T], vb: &[T]) -> f32;
}

#[derive(Default)]
pub struct DistCosine;

impl Distance<f32> for DistCosine {
    fn eval(&self, va: &[f32], vb: &[f32]) -> f32 {
        let zero: f32 = 0.;
        let res = va
            .iter()
            .zip(vb.iter())
            .map(|t| {
                (
                    (*t.0 * *t.1) as f32,
                    (*t.0 * *t.0) as f32,
                    (*t.1 * *t.1) as f32,
                )
            })
            .fold((0., 0., 0.), |acc, t| {
                (acc.0 + t.0, acc.1 + t.1, acc.2 + t.2)
            });

        if res.1 > zero && res.2 > zero {
            1. - res.0 / (res.1 * res.2).sqrt()
        } else {
            0.
        }
    }
}

#[derive(Default)]
pub struct DistDot;

impl Distance<f32> for DistDot {
    fn eval(&self, va: &[f32], vb: &[f32]) -> f32 {
        let dot = va
            .iter()
            .zip(vb.iter())
            .map(|t| (*t.0 * *t.1) as f32)
            .fold(0., |acc, t| (acc + t));
        assert!(dot <= 1.);

        1. - dot
    }
}

/// This structure is to let user define their own distance with closures.
pub struct DistFn<T: Copy + Clone + Sized + Send + Sync> {
    dist_function: Box<dyn Fn(&[T], &[T]) -> f32 + Send + Sync>,
}

impl<T: Copy + Clone + Sized + Send + Sync> DistFn<T> {
    /// construction of a DistFn
    pub fn new(f: Box<dyn Fn(&[T], &[T]) -> f32 + Send + Sync>) -> Self {
        DistFn { dist_function: f }
    }
}

impl<T: Copy + Clone + Sized + Send + Sync> Distance<T> for DistFn<T> {
    fn eval(&self, va: &[T], vb: &[T]) -> f32 {
        (self.dist_function)(va, vb)
    }
}

/// This structure uses a Rust function pointer to define the distance.
/// For commodity it can build upon a function returning a f64.
/// Beware that if F is f64, the distance converted to f32 can overflow!

#[derive(Copy, Clone)]
pub struct DistPtr<T: Copy + Clone + Sized + Send + Sync, F: Float> {
    dist_function: fn(&[T], &[T]) -> F,
}

impl<T: Copy + Clone + Sized + Send + Sync, F: Float> DistPtr<T, F> {
    /// construction of a DistPtr
    pub fn new(f: fn(&[T], &[T]) -> F) -> Self {
        DistPtr { dist_function: f }
    }
}

/// beware that if F is f64, the distance converted to f32 can overflow!
impl<T: Copy + Clone + Sized + Send + Sync, F: Float> Distance<T> for DistPtr<T, F> {
    fn eval(&self, va: &[T], vb: &[T]) -> f32 {
        (self.dist_function)(va, vb).to_f32().unwrap()
    }
}

pub fn l2_normalize(va: &mut [f32]) {
    let l2norm = va.iter().map(|t| (*t * *t) as f32).sum::<f32>().sqrt();
    if l2norm > 0. {
        for i in 0..va.len() {
            va[i] = va[i] / l2norm;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_to_dist_cos() {
        let distcos = DistCosine;
        //
        let v1: Vec<f32> = vec![1.0, -1.0, 1.0];
        let v2: Vec<f32> = vec![2.0, 1.0, -1.0];

        let d1 = Distance::eval(&distcos, &v1, &v2);
        assert_eq!(d1, 1. as f32);
        //
        let v1: Vec<f32> = vec![1.234, -1.678, 1.367];
        let v2: Vec<f32> = vec![4.234, -6.678, 10.367];
        let d1 = Distance::eval(&distcos, &v1, &v2);

        let mut normv1 = 0.;
        let mut normv2 = 0.;
        let mut prod = 0.;
        for i in 0..v1.len() {
            prod += v1[i] * v2[i];
            normv1 += v1[i] * v1[i];
            normv2 += v2[i] * v2[i];
        }
        let dcos = 1. - prod / (normv1 * normv2).sqrt();
        println!("dist cos avec macro = {:?} ,  avec for {:?}", d1, dcos);
    }

    #[test]
    fn test_dot_distances() {
        let mut v1: Vec<f32> = vec![1.234, -1.678, 1.367];
        let mut v2: Vec<f32> = vec![4.234, -6.678, 10.367];

        let mut normv1 = 0.;
        let mut normv2 = 0.;
        let mut prod = 0.;
        for i in 0..v1.len() {
            prod += v1[i] * v2[i];
            normv1 += v1[i] * v1[i];
            normv2 += v2[i] * v2[i];
        }
        let dcos = 1. - prod / (normv1 * normv2).sqrt();
        //
        l2_normalize(&mut v1);
        l2_normalize(&mut v2);

        println!(" after normalisation v1 = {:?}", v1);

        let dot = DistDot.eval(&v1, &v2);

        println!(
            "dot  cos avec prenormalisation  = {:?} ,  avec for {:?}",
            dot, dcos
        );
    }

    // #[test]
    // fn test_my_closure() {
    //     let weight = vec![0.1, 0.8, 0.1];
    //     let my_fn =  move | va : &[f32] , vb: &[f32] |  -> f32  {
    //         // should check that we work with same size for va, vb, and weight...
    //         let mut dist : f32 =  0.;
    //         for i in 0..va.len() {
    //             dist += weight[i] * (va[i] - vb[i]).abs();
    //         }
    //         dist
    //     };
    //     let my_boxed_f = Box::new(my_fn);
    //     let my_boxed_dist  = DistFn::<f32>::new(my_boxed_f);
    //     let va : Vec::<f32> = vec! [1. , 2., 3.];
    //     let vb : Vec::<f32> = vec! [2. , 2., 4.];
    //     let dist = my_boxed_dist.eval(&va, &vb);
    //     println!("test_my_closure computed : {:?}", dist);
    //     // try allocation Hnsw
    //     let _hnsw = Hnsw::<f32, DistFn<f32> >::new(10, 3, 100, 16, my_boxed_dist);
    //     //
    //     assert_eq!(dist, 0.2);
    // }
}

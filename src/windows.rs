use crate::Sample;

/// Different window functions that can be used to window the sinc function.
#[derive(Debug, Clone, Copy)]
pub enum WindowFunction {
    /// Blackman. Intermediate rolloff and intermediate attenuation.
    Blackman,
    /// Squared Blackman. Slower rolloff but better attenuation than Blackman.
    Blackman2,
    /// Blackman-Harris. Slow rolloff but good attenuation.
    BlackmanHarris,
    /// Squared Blackman-Harris. Slower rolloff but better attenuation than Blackman-Harris.
    BlackmanHarris2,
    /// Hann. Fast rolloff but not very high attenuation.
    Hann,
    /// Squared Hann. Slower rolloff and higher attenuation than simple Hann.
    Hann2,
}

/// Helper function. Standard Blackman-Harris window
pub fn blackman_harris<T>(npoints: usize) -> Vec<T>
where
    T: Sample,
{
    trace!("Making a BlackmanHarris windows with {} points", npoints);
    let mut window = vec![T::zero(); npoints];
    let pi2 = T::coerce(2.0) * T::PI;
    let pi4 = T::coerce(4.0) * T::PI;
    let pi6 = T::coerce(6.0) * T::PI;
    let np_f = T::coerce(npoints);
    let a = T::coerce(0.35875);
    let b = T::coerce(0.48829);
    let c = T::coerce(0.14128);
    let d = T::coerce(0.01168);
    for (x, item) in window.iter_mut().enumerate() {
        let x_float = T::coerce(x);
        *item = a - b * (pi2 * x_float / np_f).cos() + c * (pi4 * x_float / np_f).cos()
            - d * (pi6 * x_float / np_f).cos();
    }
    window
}

/// Helper function. Standard Blackman window
pub fn blackman<T>(npoints: usize) -> Vec<T>
where
    T: Sample,
{
    trace!("Making a Blackman windows with {} points", npoints);
    let mut window = vec![T::zero(); npoints];
    let pi2 = T::coerce(2.0) * T::PI;
    let pi4 = T::coerce(4.0) * T::PI;
    let np_f = T::coerce(npoints);
    let a = T::coerce(0.42);
    let b = T::coerce(0.5);
    let c = T::coerce(0.08);
    for (x, item) in window.iter_mut().enumerate() {
        let x_float = T::coerce(x);
        *item = a - b * (pi2 * x_float / np_f).cos() + c * (pi4 * x_float / np_f).cos();
    }
    window
}

/// Standard Hann window
pub fn hann<T>(npoints: usize) -> Vec<T>
where
    T: Sample,
{
    trace!("Making a Hann windows with {} points", npoints);
    let mut window = vec![T::zero(); npoints];
    let pi2 = T::coerce(2.0) * T::PI;
    let np_f = T::coerce(npoints);
    let a = T::coerce(0.5);
    for (x, item) in window.iter_mut().enumerate() {
        let x_float = T::coerce(x);
        *item = a - a * (pi2 * x_float / np_f).cos();
    }
    window
}

/// Make the selected window function
pub fn make_window<T>(npoints: usize, windowfunc: WindowFunction) -> Vec<T>
where
    T: Sample,
{
    let mut window = match windowfunc {
        WindowFunction::BlackmanHarris | WindowFunction::BlackmanHarris2 => {
            blackman_harris::<T>(npoints)
        }
        WindowFunction::Blackman | WindowFunction::Blackman2 => blackman::<T>(npoints),
        WindowFunction::Hann | WindowFunction::Hann2 => hann::<T>(npoints),
    };
    match windowfunc {
        WindowFunction::Blackman2 | WindowFunction::BlackmanHarris2 | WindowFunction::Hann2 => {
            window.iter_mut().for_each(|y| *y = *y * *y);
        }
        _ => {}
    };
    window
}

/// Calculate a suitable relative cutoff frequency for the given sinc length using the given window function.
/// The result is based on an approximation, which gives good results for sinc lengths from 32 to 2048.
pub fn calculate_cutoff<T>(npoints: usize, windowfunc: WindowFunction) -> T
where
    T: Sample,
{
    let (k1, k2, k3) = match windowfunc {
        WindowFunction::BlackmanHarris => (
            T::coerce(8.035953378672037),
            T::coerce(57.03078027502588),
            T::coerce(867.9402989951352),
        ),
        WindowFunction::BlackmanHarris2 => (
            T::coerce(13.75199169984904),
            T::coerce(121.68057131936176),
            T::coerce(5957.651558218036),
        ),
        WindowFunction::Blackman => (
            T::coerce(6.187398036770492),
            T::coerce(16.109602892482037),
            T::coerce(715.9711791020756),
        ),
        WindowFunction::Blackman2 => (
            T::coerce(9.542238688779452),
            T::coerce(75.81202588432767),
            T::coerce(1572.1620695552645),
        ),
        WindowFunction::Hann => (
            T::coerce(3.3520600262878313),
            T::coerce(10.446229596405484),
            T::coerce(64.84675682879767),
        ),
        WindowFunction::Hann2 => (
            T::coerce(5.403705704263967),
            T::coerce(28.227298602817687),
            T::coerce(215.34865018641966),
        ),
    };
    let one = T::one();
    one / (k1 / T::coerce(npoints)
        + k2 / T::coerce(npoints.pow(2))
        + k3 / T::coerce(npoints.pow(3))
        + one)
}

#[cfg(test)]
mod tests {
    extern crate approx;
    use crate::windows::blackman;
    use crate::windows::blackman_harris;
    use crate::windows::calculate_cutoff;
    use crate::windows::hann;
    use crate::windows::make_window;
    use crate::windows::WindowFunction;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_blackman_harris() {
        let wnd = blackman_harris::<f64>(16);
        assert_abs_diff_eq!(wnd[8], 1.0, epsilon = 0.000001);
        assert!(wnd[0] < 0.001);
        assert!(wnd[15] < 0.1);
    }

    #[test]
    fn test_blackman() {
        let wnd = blackman::<f64>(16);
        assert_abs_diff_eq!(wnd[8], 1.0, epsilon = 0.000001);
        assert!(wnd[0] < 0.000001);
        assert!(wnd[15] < 0.1);
    }

    #[test]
    fn test_blackman2() {
        let wnd = make_window::<f64>(16, WindowFunction::Blackman);
        let wnd2 = make_window::<f64>(16, WindowFunction::Blackman2);
        assert_abs_diff_eq!(wnd[1] * wnd[1], wnd2[1], epsilon = 0.000001);
        assert_abs_diff_eq!(wnd[4] * wnd[4], wnd2[4], epsilon = 0.000001);
        assert_abs_diff_eq!(wnd[7] * wnd[7], wnd2[7], epsilon = 0.000001);
        assert!(wnd2[1] > 0.000001);
        assert!(wnd2[4] > 0.000001);
        assert!(wnd2[7] > 0.000001);
    }

    #[test]
    fn test_hann() {
        let wnd = hann::<f64>(16);
        assert_abs_diff_eq!(wnd[8], 1.0, epsilon = 0.000001);
        assert!(wnd[0] < 0.000001);
        assert!(wnd[15] < 0.1);
    }

    #[test]
    fn test_cutoff() {
        let cutoff = calculate_cutoff::<f64>(128, WindowFunction::Blackman);
        assert_abs_diff_eq!(cutoff, 0.917, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(256, WindowFunction::Blackman);
        assert_abs_diff_eq!(cutoff, 0.957, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(128, WindowFunction::Blackman2);
        assert_abs_diff_eq!(cutoff, 0.856, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(256, WindowFunction::Blackman2);
        assert_abs_diff_eq!(cutoff, 0.922, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(128, WindowFunction::BlackmanHarris);
        assert_abs_diff_eq!(cutoff, 0.905, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(256, WindowFunction::BlackmanHarris);
        assert_abs_diff_eq!(cutoff, 0.950, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(128, WindowFunction::BlackmanHarris2);
        assert_abs_diff_eq!(cutoff, 0.833, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(256, WindowFunction::BlackmanHarris2);
        assert_abs_diff_eq!(cutoff, 0.909, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(128, WindowFunction::Hann);
        assert_abs_diff_eq!(cutoff, 0.929, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(256, WindowFunction::Hann);
        assert_abs_diff_eq!(cutoff, 0.963, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(128, WindowFunction::Hann2);
        assert_abs_diff_eq!(cutoff, 0.879, epsilon = 0.001);
        let cutoff = calculate_cutoff::<f64>(256, WindowFunction::Hann2);
        assert_abs_diff_eq!(cutoff, 0.936, epsilon = 0.001);
    }
}

use crate::traits::Scalar;

pub fn quadratic<S: Scalar>(a: S, b: S, c: S) -> Option<(S, S)> {
    let below_sqrt = b * b - S::from(4) * a * c;

    if below_sqrt < S::from(0) {
        None
    } else {
        let sqrt = below_sqrt.sqrt();
        let nb = -b;
        let a2 = S::from(2) * a;
        let low = (nb - sqrt) / a2;
        let high = (nb + sqrt) / a2;
        Some((low, high))
    }
}

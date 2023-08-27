fn main() {}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::net::IpAddr;

    proptest! {
        #[test]
        fn correctness_u32(a: u32, b: u32, c: u32) {
            reltester::eq(&a, &b, &c).unwrap();
            reltester::ord(&a, &b, &c).unwrap();
        }

        #[test]
        fn correctness_f32(a: f32, b: f32, c: f32) {
            reltester::partial_eq(&a, &b, &c).unwrap();
            reltester::partial_ord(&a, &b, &c).unwrap();
        }

        #[test]
        fn correctness_ip_address(a: IpAddr, b: IpAddr, c: IpAddr) {
            reltester::eq(&a, &b, &c).unwrap();
            reltester::ord(&a, &b, &c).unwrap();
        }

        #[test]
        fn vec_u32_is_truly_double_ended(x: Vec<u32>) {
            reltester::double_ended_iterator(x.iter()).unwrap();
        }
    }
}

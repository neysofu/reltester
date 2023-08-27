fn main() {}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;
    use std::net::IpAddr;

    #[quickcheck]
    fn correctness_u32(a: u32, b: u32, c: u32) -> bool {
        reltester::eq(&a, &b, &c).is_ok() && reltester::ord(&a, &b, &c).is_ok()
    }

    #[quickcheck]
    fn correctness_f32(a: f32, b: f32, c: f32) -> bool {
        reltester::partial_eq(&a, &b, &c).is_ok() && reltester::partial_ord(&a, &b, &c).is_ok()
    }

    #[quickcheck]
    fn correctness_ip_address(a: IpAddr, b: IpAddr, c: IpAddr) -> bool {
        reltester::eq(&a, &b, &c).is_ok() && reltester::ord(&a, &b, &c).is_ok()
    }

    #[quickcheck]
    fn vec_u32_is_truly_double_ended(x: Vec<u32>) -> bool {
        reltester::double_ended_iterator(x.iter()).is_ok()
    }
}

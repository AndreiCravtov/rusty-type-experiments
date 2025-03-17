pub mod option;
pub mod result;
mod vec;

#[cfg(test)]
mod tests {
    use crate::family_pattern::syntax::*;
    use std::ops::Add;

    fn double<T: Add<T, Output = T> + Clone>(x: T) -> T {
        x.clone() + x
    }

    fn checked_double(x: u32) -> Option<u32> {
        x.checked_add(x)
    }

    #[test]
    pub fn option_hkt() {
        // test pure-syntax
        let pure_u32 = 12_u32.pure::<Option<_>>();
        let pure_str = "some_string".pure::<Option<_>>();
        let pure_double_u32 = double::<u32>.pure::<Option<_>>();

        // test fmap, fconst, fvoid
        let fmapped = pure_u32.fmap(double);
        let fvoid_ignored = pure_str.fvoid();
        let fconst_ignored = fvoid_ignored.fconst(33_u32);

        // test ap, lift2a
        let applicative_apped = pure_double_u32.ap(fconst_ignored);
        let lifted_2a = (fmapped, applicative_apped).lift_2a(Add::add);

        // test bind
        let bind_checked_doubling = lifted_2a.bind(checked_double);

        // TODO: ....

        // test scombine, mempty

        // test foldr, foldl, foldr_map, foldl_map

        // test traverse
    }
}

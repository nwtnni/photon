macro_rules! impl_mut {
    ($macro:ident, $lhs:ty, $rhs:ty) => {
        $macro!($lhs, $rhs);
        $macro!($lhs, &$rhs);
    }
}

macro_rules! impl_all {
    ($macro:ident, $lhs:ty, $rhs:ty) => {
        $macro!($lhs, $lhs, $rhs);
        $macro!($lhs, $lhs, &$rhs);
        $macro!($lhs, &$lhs, $rhs);
        $macro!($lhs, &$lhs, &$rhs);
    }
}

macro_rules! make_impl_trait {
    (
        $impl_trait:ident,
        $trait:ident,
        $method:ident,
        $impl_trait_mut:ident,
        $trait_mut:ident,
        $method_mut:ident,
        $pat:pat => $fn:expr
    ) => {
        macro_rules! $impl_trait {
            ($output:ty, $lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait<$rhs> for $lhs {
                    type Output = $output;
                    #[inline]
                    fn $method(self, rhs: $rhs) -> Self::Output {
                        Self::Output::unpack(match (self, rhs) { $pat => $fn, })
                    }
                }
            }
        }

        macro_rules! $impl_trait_mut {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait_mut<$rhs> for $lhs {
                    #[inline]
                    fn $method_mut(&mut self, rhs: $rhs) {
                        let v = match (*self, rhs) { $pat => $fn, };
                        self.pack(v)
                    }
                }
            }
        }
    }
}

make_impl_trait!(
    impl_add_v2v,
    Add,
    add,
    impl_add_assign_v2v,
    AddAssign,
    add_assign,
    (l, r) => (l.x() + r.x(), l.y() + r.y())
);

make_impl_trait!(
    impl_sub_v2v,
    Sub,
    sub,
    impl_sub_assign_v2v,
    SubAssign,
    sub_assign,
    (l, r) => (l.x() - r.x(), l.y() - r.y())
);

make_impl_trait!(
    impl_mul_v2s,
    Mul,
    mul,
    impl_mul_assign_v2s,
    MulAssign,
    mul_assign,
    (v, s) => (v.x() * s.clone(), v.y() * s.clone())
);

make_impl_trait!(
    impl_div_v2s,
    Div,
    div,
    impl_div_assign_v2s,
    DivAssign,
    div_assign,
    (v, s) => (v.x() * s.clone(), v.y() * s.clone())
);

make_impl_trait!(
    impl_add_v3v,
    Add,
    add,
    impl_add_assign_v3v,
    AddAssign,
    add_assign,
    (l, r) => (l.x() + r.x(), l.y() + r.y(), l.z() + r.z())
);

make_impl_trait!(
    impl_sub_v3v,
    Sub,
    sub,
    impl_sub_assign_v3v,
    SubAssign,
    sub_assign,
    (l, r) => (l.x() - r.x(), l.y() - r.y(), l.z() - r.z())
);

make_impl_trait!(
    impl_mul_v3s,
    Mul,
    mul,
    impl_mul_assign_v3s,
    MulAssign,
    mul_assign,
    (v, s) => (v.x() * s.clone(), v.y() * s.clone(), v.z() * s.clone())
);

make_impl_trait!(
    impl_div_v3s,
    Div,
    div,
    impl_div_assign_v3s,
    DivAssign,
    div_assign,
    (v, s) => (v.x() * s.clone(), v.y() * s.clone(), v.z() * s.clone())
);

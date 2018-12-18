macro_rules! impl_mut {
    ($macro:ident, $lhs:ty, $rhs:ty) => {
        $macro!($lhs, $rhs);
        $macro!($lhs, &$rhs);
    }
}

macro_rules! impl_all {
    ($macro:ident, $output:ty, $lhs:ty, $rhs:ty) => {
        $macro!($output, $lhs, $rhs);
        $macro!($output, $lhs, &$rhs);
        $macro!($output, &$lhs, $rhs);
        $macro!($output, &$lhs, &$rhs);
    };
    ($macro:ident, $lhs:ty, $rhs:ty) => {
        $macro!($lhs, $lhs, $rhs);
        $macro!($lhs, $lhs, &$rhs);
        $macro!($lhs, &$lhs, $rhs);
        $macro!($lhs, &$lhs, &$rhs);
    };
}

macro_rules! make_impl_vector_trait {
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
                        Self::Output::from(match (self, rhs) { $pat => $fn, })
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
                        self.set(v)
                    }
                }
            }
        }
    }
}

make_impl_vector_trait!(
    impl_add_v2v,
    Add,
    add,
    impl_add_assign_v2v,
    AddAssign,
    add_assign,
    (l, r) => (l.x() + r.x(), l.y() + r.y())
);

make_impl_vector_trait!(
    impl_sub_v2v,
    Sub,
    sub,
    impl_sub_assign_v2v,
    SubAssign,
    sub_assign,
    (l, r) => (l.x() - r.x(), l.y() - r.y())
);

make_impl_vector_trait!(
    impl_mul_v2s,
    Mul,
    mul,
    impl_mul_assign_v2s,
    MulAssign,
    mul_assign,
    (v, s) => (v.x() * s.clone(), v.y() * s.clone())
);

make_impl_vector_trait!(
    impl_div_v2s,
    Div,
    div,
    impl_div_assign_v2s,
    DivAssign,
    div_assign,
    (v, s) => (v.x() / s.clone(), v.y() / s.clone())
);

make_impl_vector_trait!(
    impl_add_v3v,
    Add,
    add,
    impl_add_assign_v3v,
    AddAssign,
    add_assign,
    (l, r) => (l.x() + r.x(), l.y() + r.y(), l.z() + r.z())
);

make_impl_vector_trait!(
    impl_sub_v3v,
    Sub,
    sub,
    impl_sub_assign_v3v,
    SubAssign,
    sub_assign,
    (l, r) => (l.x() - r.x(), l.y() - r.y(), l.z() - r.z())
);

make_impl_vector_trait!(
    impl_mul_v3s,
    Mul,
    mul,
    impl_mul_assign_v3s,
    MulAssign,
    mul_assign,
    (v, s) => (v.x() * s.clone(), v.y() * s.clone(), v.z() * s.clone())
);

make_impl_vector_trait!(
    impl_div_v3s,
    Div,
    div,
    impl_div_assign_v3s,
    DivAssign,
    div_assign,
    (v, s) => (v.x() / s.clone(), v.y() / s.clone(), v.z() / s.clone())
);

macro_rules! impl_mul_mp {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = $output;
            fn mul(self, rhs: $rhs) -> Self::Output {
                unsafe {
                    let mut output: [N32; 4] = std::mem::uninitialized(); 
                    let rhs = [rhs[0], rhs[1], rhs[2], n32(1.0)];
                    let a = self.0.as_ptr() as *const f32;
                    let b = rhs.as_ptr() as *const f32;
                    let c = output.as_mut_ptr() as *mut f32;
                    matrixmultiply::sgemm(4, 4, 1, 1.0f32, a, 4, 1, b, 1, 4, 0.0, c, 1, 4);
                    return if output[3] == 1.0 {
                        Point3f::new(output[0], output[1], output[2])
                    } else {
                        Point3f::new(output[0], output[1], output[2]) / output[3]
                    }
                }
            }
        }
    }
}

macro_rules! impl_mul_mv {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = $output;
            fn mul(self, rhs: $rhs) -> Self::Output {
                unsafe {
                    let mut output: [N32; 4] = std::mem::uninitialized(); 
                    let rhs = [rhs[0], rhs[1], rhs[2], n32(0.0)];
                    let a = self.0.as_ptr() as *const f32;
                    let b = rhs.as_ptr() as *const f32;
                    let c = output.as_mut_ptr() as *mut f32;
                    matrixmultiply::sgemm(4, 4, 1, 1.0f32, a, 4, 1, b, 1, 4, 0.0, c, 1, 4);
                    Vec3f::new(output[0], output[1], output[2])
                }
            }
        }
    }
}

macro_rules! impl_mul_mm {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = $output;
            fn mul(self, rhs: $rhs) -> Self::Output {
                unsafe {
                    let mut output: [N32; 16] = std::mem::uninitialized(); 
                    let a = self.0.as_ptr() as *const f32;
                    let b = rhs.0.as_ptr() as *const f32;
                    let c = output.as_mut_ptr() as *mut f32;
                    matrixmultiply::sgemm(4, 4, 4, 1.0f32, a, 4, 1, b, 4, 1, 0.0, c, 4, 1);
                    Mat4::new(output)
                }
            }
        }
    }
}

macro_rules! make_impl_matrix_trait {
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
                        unsafe {
                            let mut output: [N; 16] = std::mem::uninitialized();
                            for i in 0..16 {
                                output[i] = match (i, self[i], rhs) { $pat => $fn };
                            }
                            Mat4::new(output)
                        }
                    }
                }
            }
        }

        macro_rules! $impl_trait_mut {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait_mut<$rhs> for $lhs {
                    #[inline]
                    fn $method_mut(&mut self, rhs: $rhs) {
                        for i in 0..16 {
                            self[i] = match (i, self[i], rhs) { $pat => $fn };
                        }
                    }
                }
            }
        }
    }
}

make_impl_matrix_trait!(
    impl_add_mm,
    Add,
    add,
    impl_add_assign_mm,
    AddAssign,
    add_assign,
    (i, l, r) => l + r[i]
);

make_impl_matrix_trait!(
    impl_sub_mm,
    Sub,
    sub,
    impl_sub_assign_mm,
    SubAssign,
    sub_assign,
    (i, l, r) => l - r[i]
);

make_impl_matrix_trait!(
    impl_mul_ms,
    Mul,
    mul,
    impl_mul_assign_ms,
    MulAssign,
    mul_assign,
    (_, l, s) => l * s.clone()
);

make_impl_matrix_trait!(
    impl_div_ms,
    Div,
    div,
    impl_div_assign_ms,
    DivAssign,
    div_assign,
    (_, l, s) => l / s.clone()
);

macro_rules! impl_mul_tv {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = $output;
            #[inline]
            fn mul(self, rhs: $rhs) -> Self::Output { self.mat * rhs }
        }
    }
}

macro_rules! impl_mul_tn {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = $output;
            #[inline]
            fn mul(self, rhs: $rhs) -> Self::Output {
                Normal3f::from(self.inv.transpose() * Vec3f::from(rhs))
            }
        }
    }
}

macro_rules! impl_mul_tt {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = $output;
            #[inline]
            fn mul(self, rhs: $rhs) -> Self::Output {
                Transform {
                    mat: self.mat * rhs.mat,
                    inv: rhs.inv * self.inv,
                }
            }
        }
    }
}

macro_rules! impl_mul_assign_tt {
    ($lhs:ty, $rhs:ty) => {
        impl MulAssign<$rhs> for $lhs {
            #[inline]
            fn mul_assign(&mut self, rhs: $rhs) {
                self.mat = self.mat * rhs.mat;
                self.inv = rhs.inv * self.inv;
            }
        }
    }
}

macro_rules! impl_mul_tb {
    ($output:ty, $lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = $output;
            #[inline]
            fn mul(self, rhs: $rhs) -> Self::Output {
                let mut bounds = Bounds3f::from(self * rhs.corner(0));
                for i in 1..8 { bounds.union_p_mut(&(self * rhs.corner(i))); }
                bounds
            }
        }
    }
}

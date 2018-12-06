macro_rules! impl_pairs {
    ($macro:ident, $lhs:ty, $rhs:ty) => {
        $macro!($lhs, $rhs);
        $macro!($lhs, &$rhs);
    }
}

macro_rules! impl_all_pairs {
    ($macro:ident, $lhs:ty, $rhs:ty) => {
        impl_pairs!($macro, $lhs, $rhs);
        impl_pairs!($macro, &$lhs, $rhs);
    }
}

macro_rules! impl_vv2 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {

        macro_rules! impl_trait {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait<$rhs> for $lhs {
                    type Output = Vector2<N>;
                    #[inline]
                    fn $method(self, rhs: $rhs) -> Self::Output {
                        let (x, y) = match ((self.x(), self.y()), (rhs.x(), rhs.y())) {
                            $pat => $fn,
                        };
                        Vector2 { x, y }
                    }
                }
            }
        }

        macro_rules! impl_trait_mut {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait_mut<$rhs> for $lhs {
                    #[inline]
                    fn $method_mut(&mut self, rhs: $rhs) {
                        let (x, y) = match ((self.x(), self.y()), (rhs.x(), rhs.y())) {
                            $pat => $fn,
                        };
                        *self = Vector2 { x, y }
                    }
                }
            }
        }

        impl_all_pairs!(impl_trait, Vector2<N>, Vector2<N>);
        impl_pairs!(impl_trait_mut, Vector2<N>, Vector2<N>);
    };
}

macro_rules! impl_vs2 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {

        macro_rules! impl_trait {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait<$rhs> for $lhs {
                    type Output = Vector2<N>;
                    #[inline]
                    fn $method(self, rhs: $rhs) -> Self::Output {
                        let (x, y) = match ((self.x(), self.y()), rhs.clone()) {
                            $pat => $fn,
                        };
                        Vector2 { x, y }
                    }
                }
            }
        }

        macro_rules! impl_trait_mut {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait_mut<$rhs> for $lhs {
                    #[inline]
                    fn $method_mut(&mut self, rhs: $rhs) {
                        let (x, y) = match ((self.x(), self.y()), rhs.clone()) {
                            $pat => $fn,
                        };
                        *self = Vector2 { x, y }
                    }
                }
            }
        }

        impl_all_pairs!(impl_trait, Vector2<N>, N);
        impl_pairs!(impl_trait_mut, Vector2<N>, N);
    };
}

macro_rules! impl_vv3 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {

        macro_rules! impl_trait {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait<$rhs> for $lhs {
                    type Output = Vector3<N>;
                    #[inline]
                    fn $method(self, rhs: $rhs) -> Self::Output {
                        let l = (self.x(), self.y(), self.z());
                        let r = (rhs.x(), rhs.y(), rhs.z());
                        let (x, y, z) = match (l, r) {
                            $pat => $fn,
                        };
                        Vector3 { x, y, z }
                    }
                }
            }
        }

        macro_rules! impl_trait_mut {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait_mut<$rhs> for $lhs {
                    #[inline]
                    fn $method_mut(&mut self, rhs: $rhs) {
                        let l = (self.x(), self.y(), self.z());
                        let r = (rhs.x(), rhs.y(), rhs.z());
                        let (x, y, z) = match (l, r) {
                            $pat => $fn,
                        };
                        *self = Vector3 { x, y, z }
                    }
                }
            }
        }

        impl_all_pairs!(impl_trait, Vector3<N>, Vector3<N>);
        impl_pairs!(impl_trait_mut, Vector3<N>, Vector3<N>);
    };
}

macro_rules! impl_vs3 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {

        macro_rules! impl_trait {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait<$rhs> for $lhs {
                    type Output = Vector3<N>;
                    #[inline]
                    fn $method(self, rhs: $rhs) -> Self::Output {
                        let (x, y, z) = match ((self.x(), self.y(), self.z()), rhs.clone()) {
                            $pat => $fn,
                        };
                        Vector3 { x, y, z }
                    }
                }
            }
        }
        
        macro_rules! impl_trait_mut {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait_mut<$rhs> for $lhs {
                    #[inline]
                    fn $method_mut(&mut self, rhs: $rhs) {
                        let (x, y, z) = match ((self.x(), self.y(), self.z()), rhs.clone()) {
                            $pat => $fn,
                        };
                        *self = Vector3 { x, y, z }
                    }
                }
            }
        }

        impl_all_pairs!(impl_trait, Vector3<N>, N);
        impl_pairs!(impl_trait_mut, Vector3<N>, N);
    };
}

macro_rules! impl_p2 {
    () => {
        macro_rules! make_impl_trait {
            ($trait:ident, $method:ident, $macro:ident, $output:ty, $pat:pat => $expr:expr) => {
                macro_rules! $macro {
                    ($lhs:ty, $rhs:ty) => {
                        impl<N: Num> $trait<$rhs> for $lhs {
                            type Output = $output;
                            fn $method(self, rhs: $rhs) -> Self::Output {
                                match ((self.x(), self.y()), (rhs.x(), rhs.y())) {
                                | $pat => $expr
                                }
                            }
                        }
                    }
                }
            }
        }

        make_impl_trait!(Add, add, impl_add, Point2<N>, ((x1, y1), (x2, y2)) => {
            Point2(Vector2::new(x1 + x2, y1 + y2))
        });

        make_impl_trait!(Sub, sub, impl_sub_p, Vector2<N>, ((x1, y1), (x2, y2)) => {
            Vector2::new(x1 + x2, y1 + y2)
        });

        make_impl_trait!(Sub, sub, impl_sub_v, Point2<N>, ((x1, y1), (x2, y2)) => {
            Point2(Vector2::new(x1 + x2, y1 + y2))
        });

        impl_all_pairs!(impl_add, Point2<N>, Vector2<N>);
        impl_all_pairs!(impl_sub_p, Point2<N>, Point2<N>);
        impl_all_pairs!(impl_sub_v, Point2<N>, Vector2<N>);
    }
}

macro_rules! impl_p2_add_assign {
    ($lhs:ty, $rhs:ty) => {
        impl<N: Num> AddAssign<$rhs> for $lhs {
            fn add_assign(&mut self, rhs: $rhs) {
                self.0.set(rhs.x(), rhs.y());
            }
        }
    }
}

macro_rules! impl_p2_sub_assign_v {
    ($lhs:ty, $rhs:ty) => {
        impl<N: Num> SubAssign<$rhs> for $lhs {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.set(self.x() - rhs.x(), self.y() - rhs.y())
            }
        }
    }
}

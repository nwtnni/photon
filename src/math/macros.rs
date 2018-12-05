macro_rules! impl_pairs {
    ($macro:ident, $lhs:ty, $rhs:ty) => {
        $macro!($lhs, $rhs);
        $macro!($lhs, &$rhs);
        $macro!($lhs, &mut $rhs);
    }
}

macro_rules! impl_all_pairs {
    ($macro:ident, $lhs:ty, $rhs:ty) => {
        impl_pairs!($macro, $lhs, $rhs);
        impl_pairs!($macro, &$lhs, $rhs);
        impl_pairs!($macro, &mut $lhs, $rhs);
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

macro_rules! impl_point2_add {
    ($lhs:ty, $rhs:ty) => {
        impl<N: Num> Add<$rhs> for $lhs {
            type Output = Point2<N>;
            fn add(self, rhs: $rhs) -> Self::Output {
                Point2(Vector2::new(self.x() - rhs.x(), self.y() - rhs.y()))
            }
        }

        impl<N: Num> AddAssign<$rhs> for $lhs {
            fn add_assign(&mut self, rhs: $rhs) {
                self.0.set(rhs.x(), rhs.y());
            }
        }
    }
}

macro_rules! impl_point2_sub {
    ($lhs:ty, $rhs:ty) => {
        impl<N: Num> Sub<$rhs> for $lhs {
            type Output = Vector2<N>;
            fn sub(self, rhs: $rhs) -> Self::Output {
                Vector2::new(self.x() - rhs.x(), self.y() - rhs.y())
            }
        }
    }
}

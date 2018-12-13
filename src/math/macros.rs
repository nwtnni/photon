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
                        self.set(x, y);
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
                        self.set(x, y);
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
                        self.set(x, y, z);
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
                        self.set(x, y, z);
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

        macro_rules! impl_add_assign_p2 {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> AddAssign<$rhs> for $lhs {
                    fn add_assign(&mut self, rhs: $rhs) {
                        self.set(self.x() + rhs.x(), self.y() + rhs.y());
                    }
                }
            }
        }

        macro_rules! impl_sub_assign_p2v {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> SubAssign<$rhs> for $lhs {
                    fn sub_assign(&mut self, rhs: $rhs) {
                        self.set(self.x() - rhs.x(), self.y() - rhs.y())
                    }
                }
            }
        }

        make_impl_trait!(Add, add, impl_add_p2, Point2<N>, ((x1, y1), (x2, y2)) => {
            Point2(Vector2::new(x1 + x2, y1 + y2))
        });

        make_impl_trait!(Sub, sub, impl_sub_p2p, Vector2<N>, ((x1, y1), (x2, y2)) => {
            Vector2::new(x1 - x2, y1 - y2)
        });

        make_impl_trait!(Sub, sub, impl_sub_p2v, Point2<N>, ((x1, y1), (x2, y2)) => {
            Point2(Vector2::new(x1 - x2, y1 - y2))
        });

        impl_all_pairs!(impl_add_p2, Point2<N>, Vector2<N>);
        impl_all_pairs!(impl_sub_p2p, Point2<N>, Point2<N>);
        impl_all_pairs!(impl_sub_p2v, Point2<N>, Vector2<N>);
        impl_pairs!(impl_add_assign_p2, Point2<N>, Vector2<N>);
        impl_pairs!(impl_sub_assign_p2v, Point2<N>, Vector2<N>);
    }
}

macro_rules! impl_p3 {
    () => {
        macro_rules! make_impl_trait {
            ($trait:ident, $method:ident, $macro:ident, $output:ty, $pat:pat => $expr:expr) => {
                macro_rules! $macro {
                    ($lhs:ty, $rhs:ty) => {
                        impl<N: Num> $trait<$rhs> for $lhs {
                            type Output = $output;
                            fn $method(self, rhs: $rhs) -> Self::Output {
                                match ((self.x(), self.y(), self.z()), (rhs.x(), rhs.y(), rhs.z())) {
                                | $pat => $expr
                                }
                            }
                        }
                    }
                }
            }
        }

        macro_rules! impl_add_assign_p3 {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> AddAssign<$rhs> for $lhs {
                    fn add_assign(&mut self, rhs: $rhs) {
                        self.set(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z());
                    }
                }
            }
        }

        macro_rules! impl_sub_assign_p3v {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> SubAssign<$rhs> for $lhs {
                    fn sub_assign(&mut self, rhs: $rhs) {
                        self.set(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
                    }
                }
            }
        }

        make_impl_trait!(Add, add, impl_add_p3, Point3<N>, ((x1, y1, z1), (x2, y2, z2)) => {
            Point3(Vector3::new(x1 + x2, y1 + y2, z1 + z2))
        });

        make_impl_trait!(Sub, sub, impl_sub_p3p, Vector3<N>, ((x1, y1, z1), (x2, y2, z2)) => {
            Vector3::new(x1 - x2, y1 - y2, z1 - z2)
        });

        make_impl_trait!(Sub, sub, impl_sub_p3v, Point3<N>, ((x1, y1, z1), (x2, y2, z2)) => {
            Point3(Vector3::new(x1 - x2, y1 - y2, z1 - z2))
        });

        impl_all_pairs!(impl_add_p3, Point3<N>, Vector3<N>);
        impl_all_pairs!(impl_sub_p3p, Point3<N>, Point3<N>);
        impl_all_pairs!(impl_sub_p3v, Point3<N>, Vector3<N>);
        impl_pairs!(impl_add_assign_p3, Point3<N>, Vector3<N>);
        impl_pairs!(impl_sub_assign_p3v, Point3<N>, Vector3<N>);
    }
}

macro_rules! impl_nn3 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {

        macro_rules! impl_trait {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait<$rhs> for $lhs {
                    type Output = Normal3<N>;
                    #[inline]
                    fn $method(self, rhs: $rhs) -> Self::Output {
                        let l = (self.x(), self.y(), self.z());
                        let r = (rhs.x(), rhs.y(), rhs.z());
                        let (x, y, z) = match (l, r) {
                            $pat => $fn,
                        };
                        Normal3::new(x, y, z)
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
                        self.set(x, y, z);
                    }
                }
            }
        }

        impl_all_pairs!(impl_trait, Normal3<N>, Normal3<N>);
        impl_pairs!(impl_trait_mut, Normal3<N>, Normal3<N>);
    };
}

macro_rules! impl_ns3 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {

        macro_rules! impl_trait {
            ($lhs:ty, $rhs:ty) => {
                impl<N: Num> $trait<$rhs> for $lhs {
                    type Output = Normal3<N>;
                    #[inline]
                    fn $method(self, rhs: $rhs) -> Self::Output {
                        let (x, y, z) = match ((self.x(), self.y(), self.z()), rhs.clone()) {
                            $pat => $fn,
                        };
                        Normal3::new(x, y, z)
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
                        self.set(x, y, z);
                    }
                }
            }
        }

        impl_all_pairs!(impl_trait, Normal3<N>, N);
        impl_pairs!(impl_trait_mut, Normal3<N>, N);
    };
}

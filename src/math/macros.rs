macro_rules! impl_vv2 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {
        impl<N: Num> $trait for Vector2<N> {
            type Output = Vector2<N>;
            fn $method(self, rhs: Self) -> Self::Output {
                let Vector2 { x: x1, y: y1 } = self;
                let Vector2 { x: x2, y: y2 } = rhs;
                let (x, y) = match ((x1, y1), (x2, y2)) {
                    $pat => $fn,
                };
                Vector2 { x, y }
            }
        }

        impl<N: Num> $trait<&Self> for Vector2<N> {
            type Output = Vector2<N>;
            fn $method(self, rhs: &Self) -> Self::Output {
                let Vector2 { x: x1, y: y1 } = self;
                let Vector2 { x: x2, y: y2 } = rhs;
                let (x, y) = match ((x1, y1), (*x2, *y2)) {
                    $pat => $fn,
                };
                Vector2 { x, y }
            }
        }

        impl<N: Num> $trait<&mut Self> for Vector2<N> {
            type Output = Vector2<N>;
            fn $method(self, rhs: &mut Self) -> Self::Output {
                let Vector2 { x: x1, y: y1 } = self;
                let Vector2 { x: x2, y: y2 } = rhs;
                let (x, y) = match ((x1, y1), (*x2, *y2)) {
                    $pat => $fn,
                };
                Vector2 { x, y }
            }
        }

        impl<N: Num> $trait_mut for Vector2<N> {
            fn $method_mut(&mut self, rhs: Self) {
                let Vector2 { x: x1, y: y1 } = *self;
                let Vector2 { x: x2, y: y2 } = rhs;
                let (x, y) = match ((x1, y1), (x2, y2)) {
                    $pat => $fn,
                };
                *self = Vector2 { x, y }
            }
        }

        impl<N: Num> $trait_mut<&Self> for Vector2<N> {
            fn $method_mut(&mut self, rhs: &Self) {
                let Vector2 { x: x1, y: y1 } = *self;
                let Vector2 { x: x2, y: y2 } = *rhs;
                let (x, y) = match ((x1, y1), (x2, y2)) {
                    $pat => $fn,
                };
                *self = Vector2 { x, y }
            }
        }

        impl<N: Num> $trait_mut<&mut Self> for Vector2<N> {
            fn $method_mut(&mut self, rhs: &mut Self) {
                let Vector2 { x: x1, y: y1 } = *self;
                let Vector2 { x: x2, y: y2 } = *rhs;
                let (x, y) = match ((x1, y1), (x2, y2)) {
                    $pat => $fn,
                };
                *self = Vector2 { x, y }
            }
        }
    };
}

macro_rules! impl_vs2 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {
        impl<N: Num> $trait<N> for Vector2<N> {
            type Output = Vector2<N>;
            fn $method(self, rhs: N) -> Self::Output {
                let Vector2 { x: x1, y: y1 } = self;
                let (x, y) = match ((x1, y1), rhs) {
                    $pat => $fn,
                };
                Vector2 { x, y }
            }
        }

        impl<N: Num> $trait<&N> for Vector2<N> {
            type Output = Vector2<N>;
            fn $method(self, rhs: &N) -> Self::Output {
                let Vector2 { x: x1, y: y1 } = self;
                let (x, y) = match ((x1, y1), *rhs) {
                    $pat => $fn,
                };
                Vector2 { x, y }
            }
        }

        impl<N: Num> $trait<&mut N> for Vector2<N> {
            type Output = Vector2<N>;
            fn $method(self, rhs: &mut N) -> Self::Output {
                let Vector2 { x: x1, y: y1 } = self;
                let (x, y) = match ((x1, y1), *rhs) {
                    $pat => $fn,
                };
                Vector2 { x, y }
            }
        }

        impl<N: Num> $trait_mut<N> for Vector2<N> {
            fn $method_mut(&mut self, rhs: N) {
                let Vector2 { x: x1, y: y1 } = *self;
                let (x, y) = match ((x1, y1), rhs) {
                    $pat => $fn,
                };
                *self = Vector2 { x, y }
            }
        }

        impl<N: Num> $trait_mut<&N> for Vector2<N> {
            fn $method_mut(&mut self, rhs: &N) {
                let Vector2 { x: x1, y: y1 } = *self;
                let (x, y) = match ((x1, y1), *rhs) {
                    $pat => $fn,
                };
                *self = Vector2 { x, y }
            }
        }

        impl<N: Num> $trait_mut<&mut N> for Vector2<N> {
            fn $method_mut(&mut self, rhs: &mut N) {
                let Vector2 { x: x1, y: y1 } = *self;
                let (x, y) = match ((x1, y1), *rhs) {
                    $pat => $fn,
                };
                *self = Vector2 { x, y }
            }
        }
    };
}

macro_rules! impl_vv3 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {
        impl<N: Num> $trait for Vector3<N> {
            type Output = Vector3<N>;
            fn $method(self, rhs: Self) -> Self::Output {
                let Vector3 { x: x1, y: y1, z: z1 } = self;
                let Vector3 { x: x2, y: y2, z: z2 } = rhs;
                let (x, y, z) = match ((x1, y1, z1), (x2, y2, z2)) {
                    $pat => $fn,
                };
                Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait<&Self> for Vector3<N> {
            type Output = Vector3<N>;
            fn $method(self, rhs: &Self) -> Self::Output {
                let Vector3 { x: x1, y: y1, z: z1 } = self;
                let Vector3 { x: x2, y: y2, z: z2 } = rhs;
                let (x, y, z) = match ((x1, y1, z1), (*x2, *y2, *z2)) {
                    $pat => $fn,
                };
                Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait<&mut Self> for Vector3<N> {
            type Output = Vector3<N>;
            fn $method(self, rhs: &mut Self) -> Self::Output {
                let Vector3 { x: x1, y: y1, z: z1 } = self;
                let Vector3 { x: x2, y: y2, z: z2 } = rhs;
                let (x, y, z) = match ((x1, y1, z1), (*x2, *y2, *z2)) {
                    $pat => $fn,
                };
                Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait_mut for Vector3<N> {
            fn $method_mut(&mut self, rhs: Self) {
                let Vector3 { x: x1, y: y1, z: z1 } = *self;
                let Vector3 { x: x2, y: y2, z: z2 } = rhs;
                let (x, y, z) = match ((x1, y1, z1), (x2, y2, z2)) {
                    $pat => $fn,
                };
                *self = Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait_mut<&Self> for Vector3<N> {
            fn $method_mut(&mut self, rhs: &Self) {
                let Vector3 { x: x1, y: y1, z: z1 } = *self;
                let Vector3 { x: x2, y: y2, z: z2 } = *rhs;
                let (x, y, z) = match ((x1, y1, z1), (x2, y2, z2)) {
                    $pat => $fn,
                };
                *self = Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait_mut<&mut Self> for Vector3<N> {
            fn $method_mut(&mut self, rhs: &mut Self) {
                let Vector3 { x: x1, y: y1, z: z1 } = *self;
                let Vector3 { x: x2, y: y2, z: z2 } = *rhs;
                let (x, y, z) = match ((x1, y1, z1), (x2, y2, z2)) {
                    $pat => $fn,
                };
                *self = Vector3 { x, y, z}
            }
        }
    };
}

macro_rules! impl_vs3 {
    ($trait:ident, $trait_mut:ident, $method:ident, $method_mut:ident, $pat:pat => $fn:expr) => {
        impl<N: Num> $trait<N> for Vector3<N> {
            type Output = Vector3<N>;
            fn $method(self, rhs: N) -> Self::Output {
                let Vector3 { x, y, z } = self;
                let (x, y, z) = match ((x, y, z), rhs) {
                    $pat => $fn,
                };
                Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait<&N> for Vector3<N> {
            type Output = Vector3<N>;
            fn $method(self, rhs: &N) -> Self::Output {
                let Vector3 { x, y, z } = self;
                let (x, y, z) = match ((x, y, z), *rhs) {
                    $pat => $fn,
                };
                Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait<&mut N> for Vector3<N> {
            type Output = Vector3<N>;
            fn $method(self, rhs: &mut N) -> Self::Output {
                let Vector3 { x, y, z } = self;
                let (x, y, z) = match ((x, y, z), *rhs) {
                    $pat => $fn,
                };
                Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait_mut<N> for Vector3<N> {
            fn $method_mut(&mut self, rhs: N) {
                let Vector3 { x, y, z } = *self;
                let (x, y, z) = match ((x, y, z), rhs) {
                    $pat => $fn,
                };
                *self = Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait_mut<&N> for Vector3<N> {
            fn $method_mut(&mut self, rhs: &N) {
                let Vector3 { x, y, z } = *self;
                let (x, y, z) = match ((x, y, z), *rhs) {
                    $pat => $fn,
                };
                *self = Vector3 { x, y, z }
            }
        }

        impl<N: Num> $trait_mut<&mut N> for Vector3<N> {
            fn $method_mut(&mut self, rhs: &mut N) {
                let Vector3 { x, y, z } = *self;
                let (x, y, z) = match ((x, y, z), *rhs) {
                    $pat => $fn,
                };
                *self = Vector3 { x, y, z }
            }
        }
    };
}

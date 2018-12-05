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

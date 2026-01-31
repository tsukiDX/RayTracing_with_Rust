use crate::math::array::Array;
use crate::math::vector::Vector2;
use crate::math::vector::Vector3;

pub struct Xorshift {}
pub struct Vnoise {}
pub struct Gnoise {}
pub struct Perlin {}


fn hermite3(f: f32) -> f32 {
    f * f * (3. - 2. * f)
}

fn hermite5(f: f32) -> f32 {
    f * f * f * (10. - 15. * f + 6. * f * f)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a - (a - b) * t
}

#[allow(dead_code)]
impl Xorshift {

    const K1: u32 = 0x456789ab;
    const K3: [u32; 3] = [0x456789ab, 0x6789ab45, 0x89ab4567];
    const U: [u32; 3] = [1, 2, 3];

    fn uhash11(mut n: u32) -> u32 {
        n ^= n.wrapping_shl(1);
        n ^= n.wrapping_shr(1);
        n = n.wrapping_mul(Self::K1);
        n ^= n.wrapping_shl(1);
        n.wrapping_mul(Self::K1)
    }

    fn uhash22(mut nx: u32, mut ny: u32, r: &mut [u32; 2]) {
        let mut tmp : [u32; 2] = [nx, ny];
        tmp[0] ^= ny.wrapping_shl(Self::U[0]);
        tmp[1] ^= nx.wrapping_shl(Self::U[1]);

        nx = tmp[0];
        ny = tmp[1];

        tmp[0] ^= ny.wrapping_shr(Self::U[0]);
        tmp[1] ^= nx.wrapping_shr(Self::U[1]);

        nx = tmp[0].wrapping_mul(Self::K3[0]);
        ny = tmp[1].wrapping_mul(Self::K3[1]);
        
        tmp[0] ^= ny.wrapping_shl(Self::U[0]);
        tmp[1] ^= nx.wrapping_shl(Self::U[1]);

        r[0] = tmp[0].wrapping_mul(Self::K3[0]);
        r[1] = tmp[1].wrapping_mul(Self::K3[1]);
    }

    fn uhash33(mut nx: u32, mut ny: u32, mut nz: u32, r: &mut [u32; 3]) {
        let mut tmp : [u32; 3] = [nx, ny, nz];
        tmp[0] ^= ny.wrapping_shl(Self::U[0]);
        tmp[1] ^= nz.wrapping_shl(Self::U[1]);
        tmp[2] ^= nx.wrapping_shl(Self::U[2]);

        nx = tmp[0];
        ny = tmp[1];
        nz = tmp[2];

        tmp[0] ^= ny.wrapping_shr(Self::U[0]);
        tmp[1] ^= nz.wrapping_shr(Self::U[1]);
        tmp[2] ^= nx.wrapping_shr(Self::U[2]);

        nx = tmp[0].wrapping_mul(Self::K3[0]);
        ny = tmp[1].wrapping_mul(Self::K3[1]);
        nz = tmp[2].wrapping_mul(Self::K3[2]);
        
        tmp[0] ^= ny.wrapping_shl(Self::U[0]);
        tmp[1] ^= nz.wrapping_shl(Self::U[1]);
        tmp[2] ^= nx.wrapping_shl(Self::U[2]);

        r[0] = tmp[0].wrapping_mul(Self::K3[0]);
        r[1] = tmp[1].wrapping_mul(Self::K3[1]);
        r[2] = tmp[2].wrapping_mul(Self::K3[2]);
    }

    pub fn rand11(p: f32) -> f32 {
        let n = p.to_bits();
        Self::uhash11(n) as f32 / u32::MAX as f32
    }

    pub fn rand22(p: Vector2) -> Vector2 {
        let n: [u32; 2] = [p.x.to_bits(), p.y.to_bits()];
        let mut r: [u32; 2] = [0, 0];
        
        Self::uhash22(n[0], n[1], &mut r);

        Vector2 { x: r[0] as f32 / u32::MAX as f32, y: r[1] as f32 / u32::MAX as f32 }
    }

    pub fn rand33(p: Vector3) -> Vector3 {
        let n: [u32; 3] = [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
        let mut r: [u32; 3] = [0, 0, 0];
        
        Self::uhash33(n[0], n[1], n[2], &mut r);

        Vector3 { 
            x: r[0] as f32 / u32::MAX as f32, 
            y: r[1] as f32 / u32::MAX as f32, 
            z: r[2] as f32 / u32::MAX as f32 
        }
    }

    pub fn rand21(p: Vector2) -> f32 {
        let n: [u32; 2] = [p.x.to_bits(), p.y.to_bits()];
        let mut r: [u32; 2] = [0, 0];
        
        Self::uhash22(n[0], n[1], &mut r);

        r[0] as f32 / u32::MAX as f32
    }

    pub fn rand31(p: Vector3) -> f32 {
        let n: [u32; 3] = [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
        let mut r: [u32; 3] = [0, 0, 0];
        
        Self::uhash33(n[0], n[1], n[2], &mut r);

        r[0] as f32 / u32::MAX as f32
    }

}

#[allow(dead_code)]
impl Vnoise {
    pub fn rand21(p: Vector2) -> f32 {
        let nx0 = p.x.floor();
        let ny0 = p.y.floor();
        let nx1 = nx0 + 1.;
        let ny1 = ny0 + 1.;

        let v: [f32; 4] = [
            Xorshift::rand21(Vector2::new(nx0, ny0)),
            Xorshift::rand21(Vector2::new(nx1, ny0)),
            Xorshift::rand21(Vector2::new(nx0, ny1)),
            Xorshift::rand21(Vector2::new(nx1, ny1))
            ];

        let fx = hermite3(p.x.fract());
        let fy = hermite3(p.y.fract());

        lerp(lerp(v[0], v[1], fx), lerp(v[2], v[3], fx), fy)
    }

    pub fn rand31(p: Vector3) -> f32 {
        let nx0 = p.x.floor();
        let ny0 = p.y.floor();
        let nz0 = p.z.floor();
        let nx1 = nx0 + 1.;
        let ny1 = ny0 + 1.;
        let nz1 = nz0 + 1.;

        let v: [f32; 8] = [
            Xorshift::rand31(Vector3::new(nx0, ny0, nz0)),
            Xorshift::rand31(Vector3::new(nx1, ny0, nz0)), 
            Xorshift::rand31(Vector3::new(nx0, ny1, nz0)), 
            Xorshift::rand31(Vector3::new(nx1, ny1, nz0)), 
            Xorshift::rand31(Vector3::new(nx0, ny0, nz1)), 
            Xorshift::rand31(Vector3::new(nx1, ny0, nz1)), 
            Xorshift::rand31(Vector3::new(nx0, ny1, nz1)), 
            Xorshift::rand31(Vector3::new(nx1, ny1, nz1))
        ];

        let fx = hermite3(p.x.fract());
        let fy = hermite3(p.y.fract());
        let fz = hermite3(p.z.fract());

        let w: [f32; 2] = [
            lerp(lerp(v[0], v[1], fx), lerp(v[2], v[3], fx), fy), 
            lerp(lerp(v[4], v[5], fx), lerp(v[6], v[7], fx), fy)
            ];

        lerp(w[0], w[1], fz)
    }
}

#[allow(dead_code)]
impl Gnoise {
    pub fn rand21(p: Vector2) -> f32 {
        let n = p.floor();
        let f = p.fract();

        let v: [f32; 4] = [
            Vector2::dot(&((Xorshift::rand22(n) - 0.5).normalized()), &f),
            Vector2::dot(&((Xorshift::rand22(Vector2{x: 1., y: 0.} + n) - 0.5).normalized()), &(f - Vector2{x: 1., y: 0.})),
            Vector2::dot(&((Xorshift::rand22(Vector2{x: 0., y: 1.} + n) - 0.5).normalized()), &(f - Vector2{x: 0., y: 1.})),
            Vector2::dot(&((Xorshift::rand22(Vector2{x: 1., y: 1.} + n) - 0.5).normalized()), &(f - Vector2{x: 1., y: 1.}))
        ];

        let f0 = hermite5(f.x);
        let f1 = hermite5(f.y);

        0.5 * lerp(lerp(v[0], v[1], f0), lerp(v[2], v[3], f0), f1) + 0.5
    }

    pub fn rand31(p: Vector3) -> f32 {
        let n = p.floor();
        let f = p.fract();

        let v: [f32; 8] = [
            Vector3::dot(&((Xorshift::rand33((n)) - 0.5).normalized()), f),
            Vector3::dot(&((Xorshift::rand33((Vector3{x: 1., y: 0., z: 0.} + n)) - 0.5).normalized()), (f - Vector3{x: 1., y: 0., z: 0.})),
            Vector3::dot(&((Xorshift::rand33((Vector3{x: 0., y: 1., z: 0.} + n)) - 0.5).normalized()), (f - Vector3{x: 0., y: 1., z: 0.})),
            Vector3::dot(&((Xorshift::rand33((Vector3{x: 1., y: 1., z: 0.} + n)) - 0.5).normalized()), (f - Vector3{x: 1., y: 1., z: 0.})),
            Vector3::dot(&((Xorshift::rand33((Vector3{x: 0., y: 0., z: 1.} + n)) - 0.5).normalized()), (f - Vector3{x: 0., y: 0., z: 1.})),
            Vector3::dot(&((Xorshift::rand33((Vector3{x: 1., y: 0., z: 1.} + n)) - 0.5).normalized()), (f - Vector3{x: 1., y: 0., z: 1.})),
            Vector3::dot(&((Xorshift::rand33((Vector3{x: 0., y: 1., z: 1.} + n)) - 0.5).normalized()), (f - Vector3{x: 0., y: 1., z: 1.})),
            Vector3::dot(&((Xorshift::rand33((Vector3{x: 1., y: 1., z: 1.} + n)) - 0.5).normalized()), (f - Vector3{x: 1., y: 1., z: 1.})),
        ];

        let f0 = hermite5(f.x);
        let f1 = hermite5(f.y);
        let f2 = hermite5(f.z);

        let a = lerp(lerp(v[0], v[1], f0), lerp(v[2], v[3], f0), f1);
        let b = lerp(lerp(v[4], v[5], f0), lerp(v[6], v[7], f0), f1);

        0.5 * lerp(a, b, f2) + 0.5
    }
}

#[allow(dead_code)]
impl Perlin {

    const K: f32 = 0.70710678;
    const C_PI8: f32 = 0.92387953;
    const S_PI8: f32 = 0.38268343;

    fn gtable2(lattice: Vector2, p: Vector2) -> f32 {
        let n: Array<u32, 2> = Array{data: [lattice.x.to_bits(), lattice.y.to_bits()]}; 
        let mut uh: [u32; 2] = [0, 0];
        Xorshift::uhash22(n[0], n[1], &mut uh);
        let ind: u32 = uh[0] >> 29;

        let u = Self::C_PI8 * if ind < 4 {p.x} else {p.y};
        let v = Self::S_PI8 * if ind < 4 {p.y} else {p.x};
        
        (if (ind & 1) == 0 {u}else {-u} + if (ind & 2) == 0 {v} else {-v})
    }

    pub fn rand21(p: Vector2) -> f32 {
        let n = p.floor();
        let f = p.fract();
        
        let v:[f32; 4] = [
            Self::gtable2(n, f), 
            Self::gtable2(n + Vector2{x: 1., y: 0.}, f - Vector2{x: 1., y: 0.}),
            Self::gtable2(n + Vector2{x: 0., y: 1.}, f - Vector2{x: 0., y: 1.}),
            Self::gtable2(n + Vector2{x: 1., y: 1.}, f - Vector2{x: 1., y: 1.}),
        ];

        let f0 = hermite5(f.x);
        let f1 = hermite5(f.y);

        0.5 * lerp(lerp(v[0], v[1], f0), lerp(v[2], v[3], f0), f1) + 0.5
    }

    fn gtable3(lattice: Vector3, p: Vector3) -> f32 {
        let n: Array<u32, 3> = Array{data: [lattice.x.to_bits(), lattice.y.to_bits(), lattice.z.to_bits()]}; 
        let mut uh: [u32; 3] = [0, 0, 0];
        Xorshift::uhash33(n[0], n[1], n[2], &mut uh);
        let ind: u32 = uh[0] >> 28;

        let u = if ind < 8 {p.x} else {p.y};
        let v = if ind < 4 {p.y} else {
            if ind == 12 || ind == 14 {p.x} else {p.z}
        };
        
        (if ind & 1 == 0 {u}else {-u} + if ind & 2 == 0 {v} else {-v})
    }

    pub fn rand31(p: Vector3) -> f32 {
        let n = p.floor();
        let f = p.fract();
        
        let v:[f32; 8] = [
            Self::gtable3(n, f) * Self::K, 
            Self::gtable3(n + Vector3{x: 1., y: 0., z: 0.}, f - Vector3{x: 1., y: 0., z: 0.}) * Self::K,
            Self::gtable3(n + Vector3{x: 0., y: 1., z: 0.}, f - Vector3{x: 0., y: 1., z: 0.}) * Self::K,
            Self::gtable3(n + Vector3{x: 1., y: 1., z: 0.}, f - Vector3{x: 1., y: 1., z: 0.}) * Self::K,
            Self::gtable3(n + Vector3{x: 0., y: 0., z: 1.}, f - Vector3{x: 0., y: 0., z: 1.}) * Self::K,
            Self::gtable3(n + Vector3{x: 1., y: 0., z: 1.}, f - Vector3{x: 1., y: 0., z: 1.}) * Self::K,
            Self::gtable3(n + Vector3{x: 0., y: 1., z: 1.}, f - Vector3{x: 0., y: 1., z: 1.}) * Self::K,
            Self::gtable3(n + Vector3{x: 1., y: 1., z: 1.}, f - Vector3{x: 1., y: 1., z: 1.}) * Self::K,
        ];

        let f0 = hermite5(f.x);
        let f1 = hermite5(f.y);
        let f2 = hermite5(f.z);

        let a = lerp(lerp(v[0], v[1], f0), lerp(v[2], v[3], f0), f1);
        let b = lerp(lerp(v[4], v[5], f0), lerp(v[6], v[7], f0), f1);

        0.5 * lerp(a, b, f2) + 0.5
    }
}
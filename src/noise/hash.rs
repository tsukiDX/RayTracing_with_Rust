pub struct Xorshift {}


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

    pub fn rand22(px: f32, py: f32, rx: &mut f32, ry: &mut f32) {
        let n: [u32; 2] = [px.to_bits(), py.to_bits()];
        let mut r: [u32; 2] = [0, 0];
        
        Self::uhash22(n[0], n[1], &mut r);

        *rx = r[0] as f32 / u32::MAX as f32;
        *ry = r[1] as f32 / u32::MAX as f32;
    }

    pub fn rand33(px: f32, py: f32, pz: f32, rx: &mut f32, ry: &mut f32, rz: &mut f32) {
        let n: [u32; 3] = [px.to_bits(), py.to_bits(), pz.to_bits()];
        let mut r: [u32; 3] = [0, 0, 0];
        
        Self::uhash33(n[0], n[1], n[2], &mut r);

        *rx = r[0] as f32 / u32::MAX as f32;
        *ry = r[1] as f32 / u32::MAX as f32;
        *rz = r[2] as f32 / u32::MAX as f32;
    }

    pub fn rand21(px: f32, py: f32) -> f32 {
        let n: [u32; 2] = [px.to_bits(), py.to_bits()];
        let mut r: [u32; 2] = [0, 0];
        
        Self::uhash22(n[0], n[1], &mut r);

        r[0] as f32 / u32::MAX as f32
    }

    pub fn rand31(px: f32, py: f32, pz: f32) -> f32 {
        let n: [u32; 3] = [px.to_bits(), py.to_bits(), pz.to_bits()];
        let mut r: [u32; 3] = [0, 0, 0];
        
        Self::uhash33(n[0], n[1], n[2], &mut r);

        r[0] as f32 / u32::MAX as f32
    }

}
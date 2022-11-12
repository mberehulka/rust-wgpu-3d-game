pub struct Cursor {
    b: Vec<u8>,
    i: usize
}
#[allow(dead_code)]
impl Cursor {
    pub fn new(b: Vec<u8>) -> Self {
        Self {
            b, i: 0
        }
    }
    pub fn read_u8(&mut self) -> u8 {
        let v = self.b[self.i];
        self.i += 1;
        v
    }
    pub fn read_u32(&mut self) -> u32 {
        let i = self.i;
        self.i += 4;
        u32::from_be_bytes([self.b[i],self.b[i+1],self.b[i+2],self.b[i+3]])
    }
    pub fn read_vec2(&mut self) -> [f32;2] {
        let i = self.i;
        self.i += 8;
        [
            f32::from_be_bytes([self.b[i],self.b[i+1],self.b[i+2],self.b[i+3]]),
            f32::from_be_bytes([self.b[i+4],self.b[i+5],self.b[i+6],self.b[i+7]])
        ]
    }
    pub fn read_vec3(&mut self) -> [f32;3] {
        let i = self.i;
        self.i += 12;
        [
            f32::from_be_bytes([self.b[i],self.b[i+1],self.b[i+2],self.b[i+3]]),
            f32::from_be_bytes([self.b[i+4],self.b[i+5],self.b[i+6],self.b[i+7]]),
            f32::from_be_bytes([self.b[i+8],self.b[i+9],self.b[i+10],self.b[i+11]])
        ]
    }
    pub fn read_vec4(&mut self) -> [f32;4] {
        let i = self.i;
        self.i += 16;
        [
            f32::from_be_bytes([self.b[i],self.b[i+1],self.b[i+2],self.b[i+3]]),
            f32::from_be_bytes([self.b[i+4],self.b[i+5],self.b[i+6],self.b[i+7]]),
            f32::from_be_bytes([self.b[i+8],self.b[i+9],self.b[i+10],self.b[i+11]]),
            f32::from_be_bytes([self.b[i+12],self.b[i+13],self.b[i+14],self.b[i+15]])
        ]
    }
    pub fn read_joints(&mut self) -> [u32;4] {
        let i = self.i;
        self.i += 4;
        [self.b[i]as u32, self.b[i+1]as u32, self.b[i+2]as u32, self.b[i+3]as u32]
    }
    pub fn read_str(&mut self) -> String {
        let mut res = Vec::<u8>::new();
        while self.b[self.i] != b'#' {
            res.push(self.b[self.i]);
            self.i += 1;
        }
        self.i += 1;
        String::from_utf8_lossy(&res).to_string()
    }
    pub fn read_mat3x3(&mut self) -> [[f32;3];3] {
        let i = self.i;
        self.i += 36;
        [
            [
                f32::from_be_bytes([self.b[i],self.b[i+1],self.b[i+2],self.b[i+3]]),
                f32::from_be_bytes([self.b[i+4],self.b[i+5],self.b[i+6],self.b[i+7]]),
                f32::from_be_bytes([self.b[i+8],self.b[i+9],self.b[i+10],self.b[i+11]])
            ],
            [
                f32::from_be_bytes([self.b[i+12],self.b[i+13],self.b[i+14],self.b[i+15]]),
                f32::from_be_bytes([self.b[i+16],self.b[i+17],self.b[i+18],self.b[i+19]]),
                f32::from_be_bytes([self.b[i+20],self.b[i+21],self.b[i+22],self.b[i+23]])
            ],
            [
                f32::from_be_bytes([self.b[i+24],self.b[i+25],self.b[i+26],self.b[i+27]]),
                f32::from_be_bytes([self.b[i+28],self.b[i+29],self.b[i+30],self.b[i+31]]),
                f32::from_be_bytes([self.b[i+32],self.b[i+33],self.b[i+34],self.b[i+35]])
            ]
        ]
    }
    pub fn read_mat4x4(&mut self) -> [[f32;4];4] {
        let i = self.i;
        self.i += 64;
        [
            [
                f32::from_be_bytes([self.b[i],self.b[i+1],self.b[i+2],self.b[i+3]]),
                f32::from_be_bytes([self.b[i+4],self.b[i+5],self.b[i+6],self.b[i+7]]),
                f32::from_be_bytes([self.b[i+8],self.b[i+9],self.b[i+10],self.b[i+11]]),
                f32::from_be_bytes([self.b[i+12],self.b[i+13],self.b[i+14],self.b[i+15]])
            ],
            [
                f32::from_be_bytes([self.b[i+16],self.b[i+17],self.b[i+18],self.b[i+19]]),
                f32::from_be_bytes([self.b[i+20],self.b[i+21],self.b[i+22],self.b[i+23]]),
                f32::from_be_bytes([self.b[i+24],self.b[i+25],self.b[i+26],self.b[i+27]]),
                f32::from_be_bytes([self.b[i+28],self.b[i+29],self.b[i+30],self.b[i+31]])
            ],
            [
                f32::from_be_bytes([self.b[i+32],self.b[i+33],self.b[i+34],self.b[i+35]]),
                f32::from_be_bytes([self.b[i+36],self.b[i+37],self.b[i+38],self.b[i+39]]),
                f32::from_be_bytes([self.b[i+40],self.b[i+41],self.b[i+42],self.b[i+43]]),
                f32::from_be_bytes([self.b[i+44],self.b[i+45],self.b[i+46],self.b[i+47]])
            ],
            [
                f32::from_be_bytes([self.b[i+48],self.b[i+49],self.b[i+50],self.b[i+51]]),
                f32::from_be_bytes([self.b[i+52],self.b[i+53],self.b[i+54],self.b[i+55]]),
                f32::from_be_bytes([self.b[i+56],self.b[i+57],self.b[i+58],self.b[i+59]]),
                f32::from_be_bytes([self.b[i+60],self.b[i+61],self.b[i+62],self.b[i+63]])
            ]
        ]
    }
}
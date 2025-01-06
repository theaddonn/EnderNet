pub struct Compression {
    level: u32,
    threshold: u32,
    method: CompressionMethod,
}

impl Compression {
    pub fn new() -> Compression {
        Compression {
            level: 0,
            threshold: 0,
            method: CompressionMethod::None,
        }
    }
    
    pub fn level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }
    
    pub fn threshold(mut self, threshold: u32) -> Self {
        self.threshold = threshold;
        self
    }
    
    pub fn method(mut self, method: CompressionMethod) -> Self {
        self.method = method;
        self
    }
}

pub enum CompressionMethod {
    None,
    Zlib,
    Snappy,
    Zstd,
}



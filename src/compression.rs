pub enum Compression {
    None,
    Zlib {
        level: u32,
        threshold: u32,},
    Snappy {
        level: u32,
        threshold: u32,},
    Zstd {
        level: u32,
        threshold: u32,},
}




pub fn clone_buffer(buffer: &[u8]) -> Vec<u8> {
    buffer.iter().map(|f| *f).collect()
}

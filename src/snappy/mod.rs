// 手动绑定C库: google/snappy
#[cfg(test)]
mod tests {
    use libc::{c_int, size_t};

    const SNAPPY_OK: c_int = 0;

    // https://github.com/google/snappy/blob/1.2.0/snappy-c.h
    // 编译时指定链接`snappy`动态链接库, 调用时会调到C编译后的二进制符号中
    // 在Rust调用C库函数时, 需要对C库的数据类型和函数签名进行封装

    #[link(name = "snappy")]
    extern "C" {
        fn snappy_compress(
            input: *const u8, // C: `const char* input` *const 指向常量的指针; C中字符实际是一个字节即u8
            input_length: size_t,
            compressed: *mut u8, // *mut 指向变量的指针
            compressed_length: *mut size_t,
        ) -> c_int;

        fn snappy_uncompress(
            compressed: *const u8,
            compressed_length: size_t,
            uncompressed: *mut u8,
            uncompressed_length: *mut size_t,
        ) -> c_int;

        fn snappy_max_compressed_length(source_length: size_t) -> size_t;

        fn snappy_uncompressed_length(
            compressed: *const u8,
            compressed_length: size_t,
            result: *mut size_t,
        ) -> c_int;

        fn snappy_validate_compressed_buffer(
            compressed: *const u8,
            compressed_length: size_t,
        ) -> c_int;
    }

    #[test]
    fn test_snappy() {
        let input = b"Hello, world!";
        let input_length = input.len() as size_t;
        let max_compressed_length = unsafe { snappy_max_compressed_length(input_length) };
        let mut compressed = vec![0; max_compressed_length];

        let mut compressed_length = max_compressed_length as size_t;
        let compress_result = unsafe {
            snappy_compress(
                input.as_ptr(), // as_ptr() 返回切片缓冲区的原始指针
                input_length,
                compressed.as_mut_ptr(),
                &mut compressed_length,
            )
        };

        println!("compressed: {:?}", compressed);
        assert_eq!(compress_result, 0);

        // 检查缓存区数据是否正确
        let status = unsafe {
            snappy_validate_compressed_buffer(compressed.as_ptr() as *const u8, compressed_length)
        };
        assert!(status == SNAPPY_OK);

        let mut uncompressed_length = 0;
        let uncompressed_length_result = unsafe {
            snappy_uncompressed_length(
                compressed.as_ptr(),
                compressed_length,
                &mut uncompressed_length,
            )
        };
        assert_eq!(uncompressed_length_result, 0);
        let mut uncompressed = vec![0; uncompressed_length as usize];
        let uncompress_result = unsafe {
            snappy_uncompress(
                compressed.as_ptr(),
                compressed_length,
                uncompressed.as_mut_ptr(),
                &mut uncompressed_length,
            )
        };

        println!(
            "uncompressed: {:?}",
            uncompressed.iter().map(|&c| c as char).collect::<String>()
        );
        assert_eq!(uncompress_result, 0);
        assert_eq!(uncompressed, input);
    }
}

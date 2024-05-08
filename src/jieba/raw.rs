#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jieba_t {
    _unused: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jieba_words_t {
    pub words: *mut *mut ::std::os::raw::c_char,
    pub length: usize,
}

extern "C" {
    pub fn jieba_new(
        dict_path: *const ::std::os::raw::c_char,
        hmm_path: *const ::std::os::raw::c_char,
        user_dict: *const ::std::os::raw::c_char,
        idf_path: *const ::std::os::raw::c_char,
        stop_word_path: *const ::std::os::raw::c_char,
    ) -> *mut jieba_t;

    pub fn jieba_free(arg1: *mut jieba_t);
    pub fn jieba_words_free(words: *mut jieba_words_t);
    pub fn jieba_cut(
        handle: *mut jieba_t,
        sentence: *const ::std::os::raw::c_char,
        is_hmm_used: ::std::os::raw::c_int,
    ) -> *mut jieba_words_t;
}

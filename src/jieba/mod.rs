use std::{
    ffi::{CStr, CString},
    path::Path,
};

use self::raw::{jieba_cut, jieba_new, jieba_words_free};

mod raw;

#[derive(Debug, Clone)]
pub struct Jieba {
    inner: *mut self::raw::jieba_t,
}

impl Jieba {
    /// Create a new instance
    pub fn new(
        dict_path: &str,
        hmm_path: &str,
        user_dict_path: &str,
        idf_path: &str,
        stop_words_path: &str,
    ) -> Self {
        let c_dict_path = CString::new(dict_path).unwrap();
        let c_hmm_path = CString::new(hmm_path).unwrap();
        let c_user_dict_path = CString::new(user_dict_path).unwrap();
        let c_idf_path = CString::new(idf_path).unwrap();
        let c_stop_words_path = CString::new(stop_words_path).unwrap();
        unsafe {
            Self {
                inner: jieba_new(
                    c_dict_path.as_ptr(),
                    c_hmm_path.as_ptr(),
                    c_user_dict_path.as_ptr(),
                    c_idf_path.as_ptr(),
                    c_stop_words_path.as_ptr(),
                ),
            }
        }
    }

    /// Create a new instance from dict data  directory
    pub fn from_dir(data_dir: &str) -> Self {
        let data_path = Path::new(data_dir);
        let dict_path = data_path.join("jieba.dict.utf8");
        let hmm_path = data_path.join("hmm_model.utf8");
        let user_dict_path = data_path.join("user.dict.utf8");
        let idf_path = data_path.join("idf.utf8");
        let stop_words_path = data_path.join("stop_words.utf8");
        Self::new(
            dict_path.to_str().unwrap(),
            hmm_path.to_str().unwrap(),
            user_dict_path.to_str().unwrap(),
            idf_path.to_str().unwrap(),
            stop_words_path.to_str().unwrap(),
        )
    }

    /// Cut the input text
    ///
    /// ## Params
    ///
    /// `text`: input text
    ///
    /// `hmm`: enable HMM or not
    pub fn cut(&self, text: &str, hmm: bool) -> Vec<String> {
        let c_text = CString::new(text).unwrap();
        let is_hmm = if hmm { 1 } else { 0 };
        unsafe {
            let ret = jieba_cut(self.inner, c_text.as_ptr(), is_hmm);
            let c_words = std::slice::from_raw_parts((*ret).words, (*ret).length);
            let words = c_words
                .into_iter()
                .map(|s| {
                    let word = CStr::from_ptr(*s);
                    word.to_string_lossy().into_owned()
                })
                .collect();
            jieba_words_free(ret);
            words
        }
    }
}

impl Drop for Jieba {
    fn drop(&mut self) {
        unsafe {
            self::raw::jieba_free(self.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Jieba;

    #[test]
    fn test_jieba() {
        let jieba = Jieba::from_dir("deps/cppjieba-cabi/cppjieba/dict");
        let words = jieba.cut("上海市南京东路", true);

        println!("{:?}", words);
        assert_eq!(vec!["上海市", "南京东路"], words);
    }
}

#![allow(non_camel_case_types)]
use cfg_if::cfg_if;

pub mod cbindgen;

cfg_if! {
    if #[cfg(feature = "snappy")] {
        pub mod snappy;
    } else if #[cfg(feature = "jieba")] {
        pub mod jieba;
    } else if #[cfg(feature = "bindgen")] {
        pub mod bindings;
    } else if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;

        fn fibonacci_iter() -> impl Iterator<Item = usize> {
            let mut state = (0, 1);
            std::iter::from_fn(move || {
                state = (state.1, state.0 + state.1);
                Some(state.0)
            })
        }

        #[wasm_bindgen]
        pub fn fibonacci(size: usize) -> Vec<usize> {
            fibonacci_iter().take(size).collect::<Vec<_>>()
        }
    } else {
        // 标准库ffi模块内置了一组实用程序, 主要用于外部函数接口FFI的绑定, 以及用在其他语言传递类C字符串的代码中
        use std::ffi::{c_char, c_uint};
        /// 关键字 `extern` 声明了这是一个外部函数, 可以被其他语言调用
        /// 使用`ABI`字符串指定调用约定, 这里是`C`调用约定
        /// 使用`no_mangle`禁止编译器混淆函数名
        #[no_mangle]
        pub unsafe extern "C" fn strlen_rs(mut char: *const c_char) -> c_uint {
            let mut len = 0;
            while *char != 0 {
                len += 1;
                char = char.offset(1);
            }

            len
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_strlen_rs() {
                // 手动创建CStr, 以终止符结尾
                let s = b"Hola, mundo!\0";
                let len = unsafe { strlen_rs(s.as_ptr() as *const c_char) };
                assert_eq!(len, s.len() as c_uint - 1);
            }

            #[test]
            fn test_abs() {
                extern "C" {
                    // 使用`link_name`指定原生库中函数或静态对象的名称
                    // 导入`<stdlib.h>`内置的`abs`函数
                    #[link_name = "abs"]
                    fn abs_in_rust(input: i32) -> i32;
                }

                assert_eq!(unsafe { abs_in_rust(-1) }, 1);
            }

            #[test]
            fn test_memory_layout() {
                /// 不同语言使用不同机制在计算机内存中布局数据, 为了保证数据在不同语言之间传递正确, 需要使用`repr(C)`属性
                /// 类型布局: 类型在内存中的排列方式, 是其数据在内存中大小、对齐方式以及字段相对偏移量.
                /// repr(C): 像C那样对类型布局, 用于结构体、枚举、联合类型
                #[repr(C)]
                struct CStruct {
                    first: i8,
                    second: i16,
                    third: i8
                }

                assert_eq!(std::mem::size_of::<CStruct>(), 6);
                assert_eq!(std::mem::align_of::<CStruct>(), 2);
            }
        }
    }
}

// FFF是这样一种机制: 用一种编程语言写的程序能调用另一种语言写的函数
// FFI有两种内涵: 正在使用的语言调用其他语言的库; 与第一种相反, 其他语言调用正在使用的语言的库

// FFI历史: 最早来自`Common Lisp`规范
// FFI现状: 不同语言称呼不同, Java中叫JNA或JNI, 有些语言叫`language binding` [Understand foreign function interface (FFI) and language binding](https://stackoverflow.com/questions/5440968/understand-foreign-function-interface-ffi-and-language-binding)
//         绑定可理解为FFI的一种实现

// FFI原理: 所有语言在编译后都以二进制形式执行, 通过一致的调用约定(ABI, 引用程序二进制接口), 将调用约定、类型表示、和名称修饰三者统一. 通过这种方式, 不同语言的二进制代码可以互相调用
//         然而计算机发展过程中出现了各类ABI规范, 如: cdecl、syscall、stdcall、fastcall、thiscall、winapi、optlink等. 庆幸的是大部分语言遵循cdecl规范

// FFI实现的困难性: 1. 带GC语言在资源管理上问题较多; 2. 复杂对象或类型, 映射时可能会出现问题; 3. 共享可变对象可能会遇到问题; 4. 如果两种语言都运行在VM中, 无法直接FFI; 5. 类型系统/对象模型/继承机制等细节, 在跨语言时成为障碍; ...

// 快速实现FFI的方式: 1. [SWIG](http://swig.org/)(简单包装界面产生器), 用于将C语言或C++写的计算机程序或函数库连接其他语言; 2. GI(GObject Introspection) 将glib/gobject生态的众多软件自动生成完整的接口描述文件, 其他语言实现Gir标准, 即可无缝调用所有经过Gir化处理的C库
//                 3. JVM平台语言之间的FFI; 4. WASM平台的FFI, WASM是一个新的字节码平台. 在设计之初就规避了JVM的问题, 目前主流语言都已实现将WASM作为编译目标

# coding: utf-8

try:
    from cffi import FFI
except ImportError:
    print("pip install cffi")

ffi = FFI()

ffi.cdef("""
    typedef enum capi_gender {
        boy,
        girl,
    } capi_gender;

    typedef struct capi_student {
        int num;
        int total;
        char name[20];
        float scores[3];
        enum capi_gender gender;
    } capi_student;

    struct capi_student *student_new(void);

    struct capi_student *student_alice(void);

    void student_free(struct capi_student *p_stu);
""")

lib = ffi.dlopen("../target/debug/libffi_demo.dylib")

py_cdata = ffi.new('capi_student *')
print('py_cdata: ', py_cdata)

student_alice = lib.student_alice()
print('name: {}, total: {}'.format(ffi.string(student_alice.name), student_alice.total))

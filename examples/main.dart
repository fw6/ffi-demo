import 'dart:ffi' as ffi;
import 'dart:io' show Platform, Directory;
import 'dart:convert';

extension CharArrayString on ffi.Array<ffi.Char> {
    String toDartString(int maxLength) {
        final codeUnits = asTypedList(maxLength).takeWhile((c) => c != 0);
        return utf8.decode(codeUnits.toList());
    }
}

extension CharTypedList on ffi.Array<ffi.Char> {
    // https://github.com/dart-lang/sdk/issues/45508
    List<int> asTypedList(int length) {
        return <int>[
            for (var i = 0; i < length; ++i) this[i],
        ];
    }
}

final class Student extends ffi.Struct {
    @ffi.Int()
    external int num;

    @ffi.Int()
    external int total;

    @ffi.Array.multi([3])
    external ffi.Array<ffi.Float> scores;

    @ffi.Array.multi([20])
    external ffi.Array<ffi.Char> name;
}

typedef StudentAliceFunc = ffi.Pointer<Student> Function();
typedef StudentAlice = ffi.Pointer<Student> Function();

void main() {
    var libraryPath = Directory.current.path + '/target/debug/libffi_demo.dylib';
    final dylib = ffi.DynamicLibrary.open(libraryPath);

    final StudentAlice student_alice = dylib
        .lookup<ffi.NativeFunction<StudentAliceFunc>>('student_alice')
        .asFunction();
    final student = student_alice().cast<Student>();

    print('Student name: ${student.ref.name.toDartString(20)}');
    print('Student total: ${student.ref.total}');
}

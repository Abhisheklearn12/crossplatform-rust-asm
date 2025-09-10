# For X86_64 bit for windows_gnu/mingw
    .text
    .globl add_two_i64
    .def add_two_i64; .scl 2; .type 32; .endef

add_two_i64:
    movq %rcx, %rax        # first arg (Windows ABI uses rcx)
    addq %rdx, %rax        # second arg (Windows ABI uses rdx)
    ret

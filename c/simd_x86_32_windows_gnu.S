# For X86_32 bit for windows_gnu/mingw
    .text
    .globl add_two_i32
    .def add_two_i32; .scl 2; .type 32; .endef

add_two_i32:
    movl 4(%esp), %eax     # first argument
    addl 8(%esp), %eax     # second argument
    ret

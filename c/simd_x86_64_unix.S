# For X86_64 bit for unix (linux/macos)
    .text
    .globl add_two_i64
    .type add_two_i64, @function

add_two_i64:
    movq %rdi, %rax        # first argument in rdi
    addq %rsi, %rax        # add second argument in rsi
    ret

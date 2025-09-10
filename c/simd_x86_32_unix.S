# For X86_32 bit for unix (linux/macos)
    .text
    .globl add_two_i32
    .type add_two_i32, @function

add_two_i32:
    movl 4(%esp), %eax     # first argument
    addl 8(%esp), %eax     # second argument
    ret

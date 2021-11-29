# x86-64 Assembly :clown::clown:

# System V calling convertion (sysvcc)
# Arguments
# 1. rdi
# 2. rsi
# 3. rdx
# 4. rcx
# 5. r8
# 6. r9
# Protected
# rbx
# rbp
# rsp
# r12 % r15
# Unprotected
# rax
# rcx
# rdx
# rdi
# rsi
# r8 % r11

.intel_syntax

.global main

.text
.code64
main:
    # @sysvcc

    # Locals
    # file FILE* [rbp - 8]
    # size uint64 [rbp - 0x10]
    # buffer byte[] [rsp]
    push %rbp
    mov %rbp, %rsp
    sub %rsp, 0x10

    lea %rsi, [%rip + TX_OPENMODE]
    lea %rdi, [%rip + TX_FILENAME]
    call fopen
    mov qword ptr [%rbp - 8], %rax

    # Get length
    mov %edx, 2
    mov %esi, 0
    mov %rdi, %rax
    call fseek
    mov %rdi, qword ptr [%rbp - 8]
    call ftell
    mov dword ptr [%rbp - 0x10], %eax
    mov %edx, 0
    mov %esi, %edx
    mov %rdi, qword ptr [%rbp - 8]
    call fseek

    # Zero-extends rax
    mov %eax, dword ptr [%rbp - 0x10]
    sub %rsp, %rax

    mov %rdi, %rsp
    mov %rcx, qword ptr [%rbp - 8]
    mov %esi, 1
    mov %edx, %eax
    call fread

    mov %rdi, qword ptr [%rbp - 8]
    call fclose

    mov %rdi, %rsp
    mov %edx, dword ptr [%rbp - 0x10]
    call solve

    mov %rax, 0
    leave
    ret

solve:
    # @sysvcc

    # Arguments
    # %rdi buffer char*
    # %rdx size uint64

    # Locals
    # total_sum uint64 [rbp - 8]
    # buffer char* [rbp - 0x10]
    # rest_size uint32 [rbp - 0x18]
    # tmp_offset uint32 [rbp - 0x14]

    push %rbp
    mov %rbp, %rsp
    sub %rsp, 0x18

    mov %rax, 0
    mov qword ptr [%rbp - 8], %rax

    mov qword ptr [%rbp - 0x10], %rdi
    mov dword ptr [%rbp - 0x18], %edx

    # Loop the tasks
    .solve.loop_begin:
        call find_blank_line
        jz .solve.end
        # Solve one line
        mov dword ptr [%rbp - 0x14], %eax
        sub %rax, 2
        mov %rdi, qword ptr [%rbp - 0x10]
        call count_group
        add qword ptr [%rbp - 8], %rax
        mov %eax, dword ptr [%rbp - 0x14]
        sub dword ptr [%rbp - 0x18], %eax
        add qword ptr [%rbp - 0x10], %rax
        mov %rdi, qword ptr [%rbp - 0x10]
        mov %edx, dword ptr [%rbp - 0x18]
        jmp .solve.loop_begin

    .solve.end:
        # Solve one more
        mov %rdi, qword ptr [%rbp - 0x10]
        # %eax is set
        call count_group
        # Print the sum
        mov %rdi, %rax
        add %rdi, qword ptr [%rbp - 8]
        call util_printd
        leave
        ret

# Count the answers in one group
count_group:
    # @sysvcc

    # Arguments
    # %rax size uint64
    # %rdi buffer char*

    # Locals
    # %rcx rest_size uint64
    # %esi set uint32 Bitset of 26 characters a-z
    # <task2>%r8d set_whole uint32</task2>
    mov %esi, 0
    mov %r8d, 0x3ffffff
    mov %rcx, %rax
    .count.loop:
        mov %al, byte ptr [%rdi]
        cmp %al, 0x0A # Newline
        je .count.handle_nl
        sub %al, 0x61
        mov %edx, 1
        movzx %eax, %al
        xchg %rax, %rcx
        shl %edx, %cl # Bitshift 1 bit into the position
        xchg %rax, %rcx
        or %esi, %edx # Store the bit
        inc %rdi
        loop .count.loop
        jmp .count.end

        .count.handle_nl:
        # <task2>
        # Flush the register
        and %r8d, %esi
        mov %esi, 0
        # </task2>
        inc %rdi
        loop .count.loop
    .count.end:
    # <task2>
    # Flush the register the last time (unless it's been already flushed)
    mov %eax, 0x3ffffff
    test %esi, %esi
    cmovz %esi, %eax
    and %r8d, %esi
    mov %esi, 0
    # </task2>
    # Count the bitset
    # <task1>
    #popcnt %eax, %esi
    # </task1> <task2>
    popcnt %eax, %r8d
    # </task2>
    ret

# Return the position of "\n\n" in the string or EOF, whichever first (implicit EOF at pos `buffer + size`)
find_blank_line:
    # @sysvcc

    # Arguments
    # %rdi string char*
    # %rdx size uint64

    # Return
    # %rax index uint64
    # @ZF eof bool

    # The index points at the start of next portion if the seperator has been found, or at the end of the buffer otherwise
    mov %rsi, 0
    dec %rsi

    dec %rdx
    jz .fbl.end
    .fbl.loop_begin:
        mov %ax, word ptr [%rdi]
        inc %rdi
        inc %rsi
        cmp %ax, 0x0A0A # "\n\n"
        je .fbl.end
        dec %rdx
        jnz .fbl.loop_begin

    .fbl.end:
        mov %rax, %rsi
        add %rax, 2
        test %rdx, %rdx
        ret

.section .rodata

.global TX_FILENAME
.global TX_OPENMODE

TX_FILENAME:
    .ascii "6.input"
    .byte 0

TX_OPENMODE:
    .ascii "r"
    .byte 0

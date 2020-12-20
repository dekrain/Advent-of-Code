// Introduction of AsFast (as possible) engine :^)

#include <fstream>
#include <string>
#include <iostream>
#include <vector>
#include <cstdint>
#include <cstdlib>
#include <cstring>

// [opcode] [execution data] [argument --]
using command_t = uint32_t;

int32_t task1(command_t* prog, intptr_t prog_size);
int32_t task2(command_t* prog, intptr_t prog_size);

// Halts when an instruction is executed twice or when execution reaches the bound of the program
// Starts execution at pc and the final value of pc is the instruction the execution halted on
int32_t execute_program(command_t* prog, intptr_t prog_size, intptr_t& pc);

int main() {
    std::ifstream file("8.input");
    std::vector<command_t> commands;
    std::string line;
    while (std::getline(file, line)) {
        // Parse command
        char opcode[4];
        std::memcpy(opcode, line.data(), 4);
        uint8_t op;
        if (opcode[0] == 'n') // "nop "
            op = 0;
        else if (opcode[0] == 'a') // "acc "
            op = 1;
        else if (opcode[0] == 'j') // "jmp "
            op = 2;
        else // invalid
            op = 0xFF;
        // Parse number
        int16_t arg = std::atoi(line.c_str() + 4);
        commands.push_back(((command_t)op << 0x18) | (command_t)(uint16_t)arg);
    }

    //std::cout << "The value is: " << task1(commands.data(), commands.size()) << '\n';
    std::cout << "The accumulator executing fixed program is " << task2(commands.data(), commands.size()) << '\n';
}

int32_t task1(command_t* prog, intptr_t prog_size) {
    intptr_t i = 0;
    return execute_program(prog, prog_size, i);
}

int32_t execute_program(command_t* prog, intptr_t prog_size, intptr_t& pc) {
    int32_t acc = 0;
    while (pc < prog_size && !(prog[pc] & 0x00'01'0000)) {
        // Mark as executed
        prog[pc] |= 0x00'01'0000;
        // Execute instruction
        switch ((prog[pc] & 0xFF'00'0000) >> 0x18) {
            case 0x00:
                // nop
                ++pc;
                break;
            case 0x01:
                // acc
                acc += (int16_t)(uint16_t)(prog[pc] & 0x00'00'FFFF);
                ++pc;
                break;
            case 0x02:
                // jmp
                pc += (int16_t)(uint16_t)(prog[pc] & 0x00'00'FFFF);
                break;
            default:
                ++pc;
                break;
        }
    }
    return acc;
}

int32_t task2(command_t* prog, intptr_t prog_size) {
    intptr_t pc = 0;
    execute_program(prog, prog_size, pc);
    for (intptr_t i = 0; i < prog_size; ++i) {
        if (prog[i] & 0x00'01'0000)
            prog[i] ^= 0x00'03'0000; // clear bit 0 and set bit 1 of data field
    }
    // Change all jmps & nops until execution reaches end
    for (intptr_t i = 0; i < prog_size; ++i) {
        if (!(prog[i] & 0x00'02'0000))
            continue;
        uint8_t prev_op = (prog[i] & 0xFF'00'0000) >> 0x18;
        if (prev_op == 0 || prev_op == 2) {
            prog[i] ^= 0x02'00'0000; // Swap the opcodes
            // Execute
            pc = 0;
            int32_t acc = execute_program(prog, prog_size, pc);
            if (pc >= prog_size) {
                // Done
                return acc;
            }
            // Clear executed flag
            for (intptr_t j = 0; j < prog_size; ++j)
                prog[j] &= ~0x00'01'0000u;
            // Restore opcode
            prog[i] = ((command_t)prev_op << 0x18) | (prog[i] & 0x00'FF'FFFF);
        }
    }
    return (int32_t)0x8000'0000;
}

def run_program(registers, program):
    def combo_operand(value: int):
        match value:
            case 0 | 1 | 2 | 3:
                return value
            case 4:
                return A
            case 5:
                return B
            case 6:
                return C

    A, B, C = registers
    pointer = 0
    outputs = []

    while pointer < len(program):
        opcode = program[pointer]
        operand = program[pointer + 1]
        print("{} {} {} {} code: {} op: {}".format(A, B, C, pointer, opcode, operand))

        if opcode == 0:  # adv
            A //= 2 ** combo_operand(operand)
        elif opcode == 1:  # bxl
            B ^= operand
        elif opcode == 2:  # bst
            B = combo_operand(operand) % 8
        elif opcode == 3:  # jnz
            if A != 0:
                pointer = operand
                continue  # skip the pointer increment
        elif opcode == 4:  # bxc
            B ^= C
        elif opcode == 5:  # out
            outputs.append(combo_operand(operand) % 8)
            return outputs
        elif opcode == 6:  # bdv
            B = A // (2 ** combo_operand(operand))
        elif opcode == 7:  # cdv
            C = A // (2 ** combo_operand(operand))

        pointer += 2

    return outputs

def part1(data):
    registers = [
        int(data[0].split(": ")[1]),
        int(data[1].split(": ")[1]),
        int(data[2].split(": ")[1]),
    ]
    program = list(map(int, data[4].split(": ")[1].split(",")))

    result = run_program(registers, program)
    return ",".join(map(str, result))


data = '''Register A: 34615120
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0
'''

if __name__ == "__main__":
    print(part1(data.splitlines()))

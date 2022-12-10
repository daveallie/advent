# frozen_string_literal: true

class CPU
  def initialize
    @registers = { x: 1 }
  end

  def get_register(name)
    @registers[name]
  end

  def run(instruction)
    return 1 if instruction == 'noop'

    @registers[:x] += instruction.split(' ')[1].to_i
    2
  end
end

class Day10
  def initialize(lines)
    @lines = lines
  end

  def part1
    cycles = 0
    cpu = CPU.new
    total = 0

    @lines.each do |line|
      last_value = cpu.get_register(:x)
      new_cycles = cpu.run(line)
      cycles += new_cycles

      (new_cycles - 1).downto(0) do |i|
        total += (cycles - i) * last_value if ((cycles - 20 - i) % 40).zero?
      end
    end

    total
  end

  def part2
    cycles = 0
    cpu = CPU.new
    output = ''

    @lines.each do |line|
      last_value = cpu.get_register(:x)
      new_cycles = cpu.run(line)
      cycles += new_cycles

      (new_cycles - 1).downto(0) do |i|
        output += (last_value..last_value + 2).cover?((cycles - i) % 40) ? '#' : '.'
        output += "\n" if ((cycles - i) % 40).zero?
      end
    end

    output
  end
end

lines = ($stdin.read || '').split("\n")
Day10.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: \n#{day.part2}"
end

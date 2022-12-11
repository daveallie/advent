# frozen_string_literal: true

class Operation
  attr_writer :common_mod

  def initialize(input, part2: false)
    tokens = input.split(' = ').last.split(' ')
    @left = tokens[0]
    @operation = tokens[1]
    @right = tokens[2]
    @part2 = part2
    @common_mod = 0
  end

  def run(input)
    if @part2
      internal_run(input) % @common_mod
    else
      (internal_run(input) / 3).floor
    end
  end

  private

  def internal_run(input)
    case @operation
    when '+'
      left(input) + right(input)
    when '*'
      left(input) * right(input)
    else
      throw 'unknown operation'
    end
  end

  def left(input)
    res = @left == 'old' ? input : @left.to_i
    return res unless @part2

    res % @common_mod
  end

  def right(input)
    res = @right == 'old' ? input : @right.to_i
    return res unless @part2

    res % @common_mod
  end
end

class Monkey
  attr_reader :inspections
  attr_reader :test_mod

  def initialize(lines, part2: false)
    @items = lines[1].split(': ').last.split(', ').map(&:to_i)
    @test_mod = lines[3].split(' ').last.to_i
    @operation = Operation.new(lines[2].split(': ').last, part2:)
    @true_monkey = lines[4].split(' ').last.to_i
    @false_monkey = lines[5].split(' ').last.to_i
    @inspections = 0
  end

  def set_common_mod(mod)
    @operation.common_mod = mod
  end

  def process_item
    return nil if @items.empty?

    @inspections += 1

    value = @items.shift
    new_value = @operation.run(value)

    [(new_value % @test_mod).zero? ? @true_monkey : @false_monkey, new_value]
  end

  def add_item(value)
    @items << value
  end
end

class Day11
  def initialize(lines)
    @lines = lines
  end

  def part1
    monkeys = @lines.slice_after('').map { |monkey_lines| Monkey.new(monkey_lines) }
    solve(monkeys, 20)
  end

  def part2
    monkeys = @lines.slice_after('').map { |monkey_lines| Monkey.new(monkey_lines, part2: true) }
    common_mod = monkeys.map(&:test_mod).reduce(:lcm)
    monkeys.each { |monkey| monkey.set_common_mod(common_mod) }

    solve(monkeys, 10000)
  end

  def solve(monkeys, loops)
    loops.times do
      monkeys.each do |monkey|
        loop do
          dest_monkey, new_value = monkey.process_item
          break if dest_monkey.nil?

          monkeys[dest_monkey].add_item(new_value)
        end
      end
    end

    monkeys.map(&:inspections).sort.last(2).reduce(:*)
  end
end

lines = ($stdin.read || '').split("\n")
Day11.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

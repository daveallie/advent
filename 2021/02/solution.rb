# frozen_string_literal: true

class Day02
  def initialize(lines)
    @lines = lines
  end

  def part1
    depth = 0
    pos = 0

    @lines.each do |line|
      instruction, amount = line.split(' ')
      amount = amount.to_i

      case instruction
      when 'forward'
        pos += amount
      when 'down'
        depth += amount
      when 'up'
        depth -= amount
      end
    end

    depth * pos
  end

  def part2
    aim = 0
    depth = 0
    pos = 0

    @lines.each do |line|
      instruction, amount = line.split(' ')
      amount = amount.to_i

      case instruction
      when 'forward'
        pos += amount
        depth += amount * aim
      when 'down'
        aim += amount
      when 'up'
        aim -= amount
      end
    end

    depth * pos
  end
end

lines = ($stdin.read || '').split("\n")
Day02.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

# frozen_string_literal: true

class Day01
  def initialize(lines)
    @lines = lines
  end

  def part1
    zeros = 0
    curr = 50
    @lines.each do |line|
      num = line[1..].to_i
      if line[0] == 'L'
        curr -= num
      else
        curr += num
      end
      curr %= 100
      zeros += 1 if curr == 0
    end
    zeros
  end

  def part2
    zeros = 0
    curr = 50
    @lines.each do |line|
      was = curr
      num = line[1..].to_i
      if line[0] == 'L'
        curr -= num
        if was == 0
          zeros += num / 100
        elsif curr <= 0
          zeros += (num - was) / 100 + 1
        end
      else
        curr += num
        zeros += curr / 100
      end

      curr %= 100
    end
    zeros
  end
end

lines = ($stdin.read || '').split("\n")
Day01.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

# frozen_string_literal: true

class Day01
  def initialize(lines)
    @elf_sums = lines.slice_after('').map do |elf_snacks|
      elf_snacks.map(&:to_i).sum
    end
  end

  def part1
    @elf_sums.max
  end

  def part2
    @elf_sums.sort[-3..].sum
  end
end

lines = ($stdin.read || '').split("\n")
Day01.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

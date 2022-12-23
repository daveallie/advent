# frozen_string_literal: true

require 'set'

Point = Struct.new(:x, :y) do
  def adjacent(dir)
    case dir
    when :n
      [
        Point.new(x - 1, y - 1),
        Point.new(x, y - 1),
        Point.new(x + 1, y - 1)
      ]
    when :s
      [
        Point.new(x - 1, y + 1),
        Point.new(x, y + 1),
        Point.new(x + 1, y + 1)
      ]
    when :w
      [
        Point.new(x - 1, y - 1),
        Point.new(x - 1, y),
        Point.new(x - 1, y + 1)
      ]
    when :e
      [
        Point.new(x + 1, y - 1),
        Point.new(x + 1, y),
        Point.new(x + 1, y + 1)
      ]
    else
      raise 'unknown dir'
    end
  end

  def move(dir)
    case dir
    when :n
      Point.new(x, y - 1)
    when :s
      Point.new(x, y + 1)
    when :w
      Point.new(x - 1, y)
    when :e
      Point.new(x + 1, y)
    else
      raise 'unknown dir'
    end
  end
end

class Day23
  def initialize(lines)
    @elves = Set.new(
      lines.flat_map.with_index do |line, y|
        line.chars.map.with_index do |char, x|
          next unless char == '#'

          Point.new(x, y)
        end.compact
      end
    )
  end

  def part1
    dirs = %i[n s w e]
    elves = @elves

    10.times do
      elves, = run(elves, dirs)
      dirs.rotate!
    end

    min_x, max_x = elves.map(&:x).minmax
    min_y, max_y = elves.map(&:y).minmax
    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.size
  end

  def part2
    dirs = %i[n s w e]
    elves = @elves
    runs = 0

    loop do
      elves, movement_count = run(elves, dirs)
      runs += 1
      break if movement_count.zero?

      dirs.rotate!
    end

    runs
  end

  private

  def run(elves, dirs)
    movements = {}
    unmoved_elves = Set.new

    elves.each do |elf|
      available_dirs = dirs.filter do |dir|
        elf.adjacent(dir).none? { |point| elves.include?(point) }
      end

      if available_dirs.empty? || available_dirs.length == 4
        unmoved_elves << elf
        next
      end

      next_point = elf.move(available_dirs.first)

      if movements.key?(next_point)
        # move to occupied, remove moved elf (if haven't already) and don't move this one
        if movements[next_point]
          unmoved_elves << movements[next_point]
          movements[next_point] = nil
        end

        unmoved_elves << elf
        next
      end

      movements[next_point] = elf
    end

    moved_elves = movements.map { |k, v| k if v }.compact
    [unmoved_elves + moved_elves, moved_elves.size]
  end
end

lines = ($stdin.read || '').split("\n")
Day23.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

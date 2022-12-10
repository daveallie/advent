# frozen_string_literal: true

require 'set'

class Point
  attr_reader :x, :y

  def initialize(x, y)
    @x = x
    @y = y
  end

  def move(dir)
    case dir
    when 'U'
      @y += 1
    when 'D'
      @y -= 1
    when 'L'
      @x -= 1
    when 'R'
      @x += 1
    end
  end

  def move_towards(other)
    return if [(x - other.x).abs, (y - other.y).abs].max <= 1

    @x += other.x > x ? 1 : -1 unless x == other.x
    @y += other.y > y ? 1 : -1 unless y == other.y
  end
end

class Day09
  def initialize(lines)
    @lines = lines
  end

  def part1
    solve(2)
  end

  def part2
    solve(10)
  end

  def solve(size)
    points = Array.new(size) { Point.new(0, 0) }
    visited = Set.new

    @lines.each do |line|
      dir, times = line.split(' ')
      times = times.to_i

      times.times do
        points.first.move(dir)
        points.each_cons(2) do |from, to|
          to.move_towards(from)
        end
        visited << [points.last.x, points.last.y]
      end
    end

    visited.count
  end
end

lines = ($stdin.read || '').split("\n")
Day09.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

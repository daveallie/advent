# frozen_string_literal: true

class Point
  attr_accessor :x, :y

  def initialize(x, y)
    @x = x
    @y = y
  end

  def move(direction)
    case direction
    when :up
      Point.new(x, y - 1)
    when :down
      Point.new(x, y + 1)
    when :left
      Point.new(x - 1, y)
    when :right
      Point.new(x + 1, y)
    end
  end

  def eql?(other)
    self == other
  end

  def ==(other)
    x == other.x && y == other.y
  end

  def hash
    [x, y].hash
  end

  def inspect
    "(#{x}, #{y})"
  end
end

class BeamPos
  attr_accessor :point, :direction

  def initialize(point, direction)
    @point = point
    @direction = direction
  end

  def next_pos(cell)
    return [move(:up), move(:down)] if cell == '|' && %i[left right].include?(direction)
    return [move(:left), move(:right)] if cell == '-' && %i[up down].include?(direction)

    if cell == '/'
      return case direction
             when :up
               [move(:right)]
             when :down
               [move(:left)]
             when :left
               [move(:down)]
             when :right
               [move(:up)]
             end
    end

    if cell == '\\'
      return case direction
             when :up
               [move(:left)]
             when :down
               [move(:right)]
             when :left
               [move(:up)]
             when :right
               [move(:down)]
             end
    end

    [move(direction)]
  end

  def move(direction)
    BeamPos.new(point.move(direction), direction)
  end

  def eql?(other)
    self == other
  end

  def ==(other)
    point == other.point && direction == other.direction
  end

  def hash
    [point, direction].hash
  end
end

class Day16
  def initialize(input)
    chars = input.split("\n").map(&:chars)
    @map = chars.each_with_index.flat_map do |row, y|
      row.each_with_index.map do |char, x|
        [Point.new(x, y), char]
      end
    end.to_h

    @bounds = Point.new(chars.first.length, chars.length)
  end

  def part1
    solve(BeamPos.new(Point.new(0, 0), :right))
  end

  def part2
    starts = [
      *(0...@bounds.x).flat_map { |x| [BeamPos.new(Point.new(x, 0), :down), BeamPos.new(Point.new(x, @bounds.y - 1), :up)] },
      *(0...@bounds.y).flat_map { |y| [BeamPos.new(Point.new(0, y), :right), BeamPos.new(Point.new(@bounds.x - 1, y), :left)] }
    ]

    starts.map { |start| solve(start) }.max
  end

  private

  def solve(start)
    seen = Set.new
    queue = [start]

    while queue.any?
      pos = queue.shift
      next if seen.include?(pos)
      next unless @map[pos.point]

      seen << pos
      queue += pos.next_pos(@map[pos.point])
    end

    seen.map(&:point).uniq.count
  end
end

input = ($stdin.read || '')
Day16.new(input).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

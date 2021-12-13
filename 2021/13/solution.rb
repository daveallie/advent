# frozen_string_literal: true

require 'set'

class Point
  attr_accessor :x, :y

  def initialize(x, y)
    self.x = x
    self.y = y
  end

  def to_s
    "(#{x}, #{y})"
  end

  def dup
    Point.new(x, y)
  end

  def eql?(other)
    other.is_a?(Point) && other.x == x && other.y == y
  end

  def hash
    [x, y].hash
  end
end

class Fold
  attr_reader :dir, :line

  def initialize(fold_def)
    details, line = fold_def.split('=')
    @dir = details[-1]
    @line = line.to_i
  end
end

class Day13
  def initialize(lines)
    @points = Set.new
    @folds = []
    ingesting_points = true
    lines.each do |line|
      if line == ''
        ingesting_points = false
        next
      end

      if ingesting_points
        @points.add(Point.new(*line.split(',').map(&:to_i)))
      else
        @folds << Fold.new(line)
      end
    end
  end

  def part1
    points = fold(@points, @folds[0])
    points.length
  end

  def part2
    points = @folds.reduce(@points) do |points, fold|
      fold(points, fold)
    end

    output(points)
    nil
  end

  private

  def fold(points, fold)
    fold_dir = fold.dir.to_sym
    new_points = Set.new

    points.each do |point|
      new_point = point.dup
      if new_point.send(fold_dir) > fold.line
        # pos - 2 * (pos - line)
        # pos - 2 * pos + 2 * line
        # 2 * line - pos
        new_point.send("#{fold_dir}=", 2 * fold.line - new_point.send(fold_dir))
      end
      new_points.add(new_point)
    end

    new_points
  end

  def output(points)
    rows = points.map(&:y).max
    cols = points.map(&:x).max

    (rows + 1).times do |row|
      (cols + 1).times do |col|
        point = points.find { |p| p.x == col && p.y == row }
        print point ? '#' : '.'
      end
      puts
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day13.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts 'Part 2:'
  day.part2
end

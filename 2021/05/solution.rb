# frozen_string_literal: true

require 'set'

class Line
  def initialize(x1, y1, x2, y2)
    @x1 = x1
    @x2 = x2
    @y1 = y1
    @y2 = y2

    @x_dir = num_dir(x1, x2)
    @y_dir = num_dir(y1, y2)
  end

  def straight?
    @x_dir.zero? || @y_dir.zero?
  end

  def points
    x = @x1
    y = @y1

    points = []

    loop do
      points << [x, y]

      break if x == @x2 && y == @y2

      x += @x_dir
      y += @y_dir
    end

    points
  end

  private

  def num_dir(first, second)
    if first == second
      0
    elsif first < second
      1
    else
      -1
    end
  end
end

class Day05
  def initialize(lines)
    @lines = lines.map { |line| Line.new(*line.split(' -> ').map { |point| point.split(',') }.flatten.map(&:to_i)) }
  end

  def part1
    seen_once = Set.new
    seen_twice = Set.new

    @lines.filter(&:straight?).each do |line|
      seen_twice += (seen_once & line.points)
      line.points.each { |p| seen_once.add(p) }
    end

    seen_twice.size
  end

  def part2
    seen_once = Set.new
    seen_twice = Set.new

    @lines.each do |line|
      seen_twice += (seen_once & line.points)
      line.points.each { |p| seen_once.add(p) }
    end

    seen_twice.size
  end
end

lines = ($stdin.read || '').split("\n")
Day05.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

# frozen_string_literal: true

class Point
  attr_accessor :row, :col

  def initialize(row, col)
    @row = row
    @col = col
  end

  def inspect
    "<Point #{@row}, #{@col}>"
  end

  def eql?(other)
    @row == other.row && @col == other.col
  end

  def ==(other)
    eql?(other)
  end

  def hash
    [@row, @col].hash
  end
end

class Num
  attr_accessor :value, :left, :right

  def initialize(value, left, right)
    @value = value
    @left = left
    @right = right
  end

  def adj_points
    res = []
    ((left.col - 1)..(right.col + 1)).each do |col|
      res << Point.new(left.row - 1, col)
      res << Point.new(left.row + 1, col)
    end
    res + [Point.new(left.row, left.col - 1), Point.new(left.row, right.col + 1)]
  end

  def adj_point?(point)
    ((left.row - 1)..(right.row + 1)).cover?(point.row) &&
      ((left.col - 1)..(right.col + 1)).cover?(point.col)
  end

  def inspect
    "<Num #{@value} #{@left.inspect} #{@right.inspect}>"
  end
end

class Day03
  def initialize(lines)
    @numbers = []
    @symbols = []
    width = lines.first.length
    left = num = nil

    lines.each_with_index do |line, row|
      if left
        @numbers << Num.new(num, left, Point.new(left.row, width - 1))
        left = nil
      end

      line.split('').each_with_index do |symbol, col|
        if ('0'..'9').cover?(symbol)
          unless left
            num = 0
            left = Point.new(row, col)
          end
          num *= 10
          num += symbol.to_i
        else
          if left
            @numbers << Num.new(num, left, Point.new(left.row, col - 1))
            left = nil
          end

          @symbols << Point.new(row, col) if symbol != '.'
        end
      end
    end
  end

  def part1
    @numbers.find_all { |number| number.adj_points.intersect?(@symbols) }
            .sum(&:value)
  end

  def part2
    @symbols.sum do |symbol|
      found = @numbers.find_all { |number| number.adj_point?(symbol) }
      found.length == 2 ? found.map(&:value).inject(:*) : 0
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day03.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

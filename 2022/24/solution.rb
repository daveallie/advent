# frozen_string_literal: true

require 'set'

Point = Struct.new(:x, :y) do
  def move(dir)
    case dir
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
end

Blizzard = Struct.new(:point, :dir) do
  def next(width, height)
    next_point = point.move(dir)

    if next_point.x < 1
      next_point.x = width
    elsif next_point.x > width
      next_point.x = 1
    elsif next_point.y < 1
      next_point.y = height
    elsif next_point.y > height
      next_point.y = 1
    end

    Blizzard.new(next_point, dir)
  end
end

State = Struct.new(:loc, :time)

class Day24
  def initialize(lines)
    @height = lines.length - 2
    @width = lines[0].length - 2
    @start = Point.new(1, 0)
    @finish = Point.new(@width, @height + 1)
    @cycle_len = @width.lcm(@height)

    # build out a map of each cycle and the blizzard occupied points
    blizzards = []
    lines.each_with_index do |line, y|
      line.chars.each_with_index do |char, x|
        point = Point.new(x, y)
        next blizzards << Blizzard.new(point, :up) if char == '^'
        next blizzards << Blizzard.new(point, :down) if char == 'v'
        next blizzards << Blizzard.new(point, :left) if char == '<'
        next blizzards << Blizzard.new(point, :right) if char == '>'
      end
    end
    @cycle_blizzards = {}
    @cycle_len.times do |i|
      @cycle_blizzards[i] = Set.new(blizzards.map(&:point))
      blizzards = blizzards.map { |b| b.next(@width, @height) }
    end
  end

  def part1
    solve(@start, @finish, 0)
  end

  def part2
    time_to_finish = solve(@start, @finish, 0)
    time_to_start = solve(@finish, @start, time_to_finish)
    solve(@start, @finish, time_to_start)
  end

  private

  def solve(start, finish, start_time)
    states = [State.new(start, start_time)]
    seen = Set.new

    while states.length
      state = states.shift
      next if seen.include?(state)

      seen << state
      next_time = state.time + 1
      next_cycle_time = next_time % @cycle_len

      %i[up down left right].each do |dir|
        next_loc = state.loc.move(dir)
        # found finish, return total time
        return next_time if next_loc == finish

        # out of bounds
        next unless (1..@width).cover?(next_loc.x) && (1..@height).cover?(next_loc.y)
        # blizzard occupied
        next if @cycle_blizzards[next_cycle_time].include?(next_loc)

        # movement option
        states << State.new(next_loc, next_time)
      end

      # wait on current tile
      states << State.new(state.loc, next_time) unless @cycle_blizzards[next_cycle_time].include?(state.loc)
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day24.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

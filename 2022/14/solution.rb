# frozen_string_literal: true

class Day14
  def initialize(lines)
    @walls = Hash.new { |h, k| h[k] = false }

    lines.each do |line|
      line.split(' -> ').map { |point| point.split(',').map(&:to_i) }.each_cons(2).each do |start, finish|
        if start[0] == finish[0]
          y1, y2 = [start[1], finish[1]].sort
          (y1..y2).each { |y| @walls[[start[0], y]] = true }
        else
          x1, x2 = [start[0], finish[0]].sort
          (x1..x2).each { |x| @walls[[x, start[1]]] = true }
        end
      end
    end

    @max_y = @walls.keys.map(&:last).max
  end

  def part1
    solve
  end

  def part2
    solve(part2: true)
  end

  private

  def solve(part2: false)
    filled = @walls.dup
    count = 0

    loop do
      # debug(filled)
      new_sand = next_sand_location(filled, [500, 0], part2:)
      break unless new_sand

      filled[new_sand] = true
      count += 1
    end

    count
  end

  def next_sand_location(filled, next_sand, part2: false)
    return nil if filled[next_sand]
    return nil if next_sand[1] > @max_y && !part2

    next_loc = [
      [next_sand[0], next_sand[1] + 1],
      [next_sand[0] - 1, next_sand[1] + 1],
      [next_sand[0] + 1, next_sand[1] + 1]
    ].find do |loc|
      !filled[loc] && loc[1] < @max_y + 2
    end

    return next_sand_location(filled, next_loc, part2:) if next_loc

    next_sand
  end

  def debug(filled)
    min_x, max_x = filled.keys.map(&:first).minmax
    max_y = filled.keys.map(&:last).max
    (0..max_y).each do |y|
      (min_x..max_x).each do |x|
        if @walls[[x, y]] || y == @max_y + 2
          print '#'
        elsif filled[[x, y]]
          print 'o'
        else
          print '.'
        end
      end
      puts
    end
    puts
  end
end

lines = ($stdin.read || '').split("\n")
Day14.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

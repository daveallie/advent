# frozen_string_literal: true

class ProbMap
  NORTH_EXIT = %w[L J |].freeze
  SOUTH_EXIT = %w[7 F |].freeze
  EAST_EXIT = %w[L F -].freeze
  WEST_EXIT = %w[J 7 -].freeze

  attr_reader :start, :width, :height

  def initialize(cells)
    @cells = cells
    @height = cells.size
    @width = cells.first.size

    find_and_replace_start
  end

  def cell(x, y)
    @cells[y] ? @cells[y][x] : nil
  end

  def neighbors(x, y)
    c = cell(x, y)

    neighs = []
    neighs << [x - 1, y] if WEST_EXIT.include?(c) && EAST_EXIT.include?(cell(x - 1, y))
    neighs << [x + 1, y] if EAST_EXIT.include?(c) && WEST_EXIT.include?(cell(x + 1, y))
    neighs << [x, y - 1] if NORTH_EXIT.include?(c) && SOUTH_EXIT.include?(cell(x, y - 1))
    neighs << [x, y + 1] if SOUTH_EXIT.include?(c) && NORTH_EXIT.include?(cell(x, y + 1))
    neighs
  end

  def poly_points
    @poly_points ||= distances.keys
  end

  def distances
    return @distances if @distances

    queue = [start]
    visited = {}
    @distances = { start => 0 }

    while queue.any?
      node = queue.shift
      x, y = node
      visited[node] = true
      neighbors(x, y).each do |neigh|
        next if visited[neigh]

        @distances[neigh] = @distances[node] + 1
        queue << neigh
      end
    end

    @distances
  end

  # debug helper
  def pretty_print(exlude_extra: false, inside_points: [])
    @cells.each_with_index.map do |row, y|
      row.join('').tr('L', '└').tr('J', '┘').tr('7', '┐').tr('F', '┌').tr('-', '─').tr('|',
                                                                                       '│').chars.each_with_index.map do |c, x|
        if inside_points.include?([x, y])
          'I'
        elsif exlude_extra && !poly_points.include?([x, y])
          '.'
        else
          c
        end
      end.join('')
    end.join("\n")
  end

  private

  def find_and_replace_start
    @cells.each_with_index do |row, y|
      row.each_with_index do |c, x|
        next unless c == 'S'

        @start = [x, y]

        # replace start with a proper symbol
        @cells[y][x] =
          if SOUTH_EXIT.include?(cell(x, y - 1)) && NORTH_EXIT.include?(cell(x, y + 1))
            '|'
          elsif EAST_EXIT.include?(cell(x - 1, y)) && WEST_EXIT.include?(cell(x + 1, y))
            '-'
          elsif SOUTH_EXIT.include?(cell(x, y - 1)) && EAST_EXIT.include?(cell(x - 1, y))
            'J'
          elsif SOUTH_EXIT.include?(cell(x, y - 1)) && WEST_EXIT.include?(cell(x + 1, y))
            'L'
          elsif NORTH_EXIT.include?(cell(x, y + 1)) && EAST_EXIT.include?(cell(x - 1, y))
            '7'
          elsif NORTH_EXIT.include?(cell(x, y + 1)) && WEST_EXIT.include?(cell(x + 1, y))
            'F'
          else
            raise 'could not determine start replacement'
          end

        return
      end
    end
  end
end

class Day10
  def initialize(lines)
    @map = ProbMap.new(lines.map(&:chars))
  end

  def part1
    @map.distances.values.max
  end

  def part2
    poly_points = @map.poly_points
    x_min, x_max = poly_points.map(&:first).minmax
    y_min, y_max = poly_points.map(&:last).minmax

    points_inside = []
    (y_min..y_max).each do |y|
      arity = false
      (x_min..x_max).each do |x|
        if poly_points.include?([x, y])
          arity = !arity if ProbMap::SOUTH_EXIT.include?(@map.cell(x, y))
          next
        end
        points_inside << [x, y] if arity
      end
    end

    points_inside.size
  end
end

lines = ($stdin.read || '').split("\n")
Day10.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

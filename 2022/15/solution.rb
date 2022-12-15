# frozen_string_literal: true

Point = Struct.new(:x, :y) do
  def dist_to(other)
    (x - other.x).abs + (y - other.y).abs
  end
end

Sensor = Struct.new(:point, :range) do
  def x_range_at_y(y)
    return point.x...point.x if y < point.y - range || y > point.y + range

    xoffset = range - (y - point.y).abs
    (point.x - xoffset)..(point.x + xoffset)
  end
end

class Day15
  def initialize(lines)
    @sensors = []
    @beacons = []

    lines.each do |line|
      line.scan(/Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)/) do |sx, sy, bx, by|
        sensor_point = Point.new(sx.to_i, sy.to_i)
        beacon_point = Point.new(bx.to_i, by.to_i)
        @sensors << Sensor.new(sensor_point, sensor_point.dist_to(beacon_point))
        @beacons << beacon_point
      end
    end

    @beacons.uniq!
  end

  def part1
    # y_target = 10
    y_target = 2_000_000

    sensor_ranges_at_y(y_target).sum(&:size) -
      @sensors.count { |s| s.point.y == y_target } -
      @beacons.count { |b| b.y == y_target }
  end

  def part2
    # search = 0..20
    search = 0..4_000_000

    result = search.lazy.map do |y|
      # filter out ranges which are within the search space
      scoped_ranges = sensor_ranges_at_y(y).filter { |r| r.begin <= search.end && r.end >= search.begin }
      # if there is only a single continuous range, then we continue to the next y
      next if scoped_ranges.length <= 1

      # there is a gap in the ranges, so take the first x coordinate after the first range
      Point.new(scoped_ranges.first.end + 1, y)
    end.reject(&:nil?).first

    result.x * 4_000_000 + result.y
  end

  private

  def sensor_ranges_at_y(y)
    ranges = @sensors
             .map { |s| s.x_range_at_y(y) }
             .filter { |r| r.size.positive? }
    combine_ranges(ranges)
  end

  def combine_ranges(ranges)
    ranges.sort_by!(&:begin)
    ranges.each_with_object([]) do |r, acc|
      if acc.empty?
        acc << r
      else
        last = acc.last

        # merge overlapping or adjacent ranges
        if r.begin <= last.end || r.begin == last.end + 1
          acc[-1] = last.begin..[last.end, r.end].max
        else
          acc << r
        end
      end
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day15.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

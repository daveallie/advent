# frozen_string_literal: true

class TMap
  attr_reader :source_range, :offset

  def initialize(dest_start, source_start, length)
    @source_range = source_start..(source_start + length - 1)
    @offset = dest_start - source_start
  end

  def source_start
    @source_range.first
  end
end

class SplitableRange < Range
  # Divides this range into three ranges, before, in, and after the given range
  def split_with(other)
    # this range is entirely before other range
    return [self, nil, nil] if last < other.first
    # this range is entirely after other range
    return [nil, nil, self] if other.last < first
    # this range is entirely contained by other range
    return [nil, self, nil] if other.first <= first && last <= other.last

    # splitting required
    before = SplitableRange.new(first, other.first - 1) if first < other.first
    contained = SplitableRange.new([first, other.first].max, [last, other.last].min)
    after = SplitableRange.new(other.last + 1, last) if other.last < last

    [before, contained, after]
  end

  def offset(offset)
    SplitableRange.new(first + offset, last + offset)
  end
end

class Day05
  def initialize(sections)
    @seeds = sections[0].split(': ').last.split(' ').map(&:to_i)

    # map groups are sorted by source_start desc
    @map_groups = sections[1..].map do |section_maps_raw|
      section_maps = section_maps_raw.split("\n")[1..]
      section_maps.map { |sm| TMap.new(*sm.split(' ').map(&:to_i)) }.sort_by(&:source_start).reverse
    end
  end

  def part1
    seed_ranges = @seeds.map { |s| SplitableRange.new(s, s) }
    solve(seed_ranges)
  end

  def part2
    seed_ranges = @seeds.each_slice(2).map { |start, range| SplitableRange.new(start, start + range - 1) }
    solve(seed_ranges)
  end

  private

  def solve(curr_ranges)
    @map_groups.each do |maps|
      next_ranges = []

      while curr_ranges.any?
        curr_range = curr_ranges.shift
        # find potential map
        map = maps.bsearch { |m| m.source_start <= curr_range.first }
        next next_ranges << curr_range unless map

        before, contained, after = curr_range.split_with(map.source_range)
        # no overlap, so just add to next ranges
        next next_ranges << curr_range unless contained

        # add before and after ranges back to pool of current ranges
        curr_ranges << before if before
        curr_ranges << after if after

        # apply map to contained range
        next_ranges << contained.offset(map.offset)
      end

      curr_ranges = next_ranges
    end

    curr_ranges.map(&:first).min
  end
end

sections = ($stdin.read || '').split("\n\n")
Day05.new(sections).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

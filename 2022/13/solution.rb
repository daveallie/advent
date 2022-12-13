# frozen_string_literal: true

require 'json'

class Packet
  include Comparable

  attr_reader :data

  def initialize(data)
    @data = Array(data)
  end

  def length
    data.length
  end

  def <=>(other)
    data.zip(other.data).each do |l, r|
      # right is nil if it has run out of items, so self is greater
      return 1 if r.nil?

      res = l.is_a?(Numeric) && r.is_a?(Numeric) ? l <=> r : Packet.new(l) <=> Packet.new(r)
      return res unless res.zero?
    end

    # all items up to length of self are equal, so if other is longer, self is less
    length < other.length ? -1 : 0
  end
end

class Day13
  def initialize(lines)
    @pairs = lines.slice_after('').map do |left, right|
      [Packet.new(JSON.parse(left)), Packet.new(JSON.parse(right))]
    end
  end

  def part1
    @pairs.map.with_index do |(left, right), index|
      left <= right ? index + 1 : 0
    end.sum
  end

  def part2
    p1 = Packet.new([[2]])
    p2 = Packet.new([[6]])
    sorted_packets = (@pairs + [p1, p2]).flatten.sort

    (sorted_packets.index(p1) + 1) * (sorted_packets.index(p2) + 1)
  end
end

lines = ($stdin.read || '').split("\n")
Day13.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

# frozen_string_literal: true

class Day08
  def initialize(lines)
    @directions = lines[0].split('')
    @adjacent = {}
    lines[2..].each do |line|
      from, tos = line.split(' = ')
      @adjacent[from] = tos[1..-2].split(', ')
    end
  end

  def part1
    dirs = @directions.cycle
    node = 'AAA'
    count = 0

    while node != 'ZZZ'
      dir = dirs.next
      node = @adjacent[node][dir == 'L' ? 0 : 1]
      count += 1
    end

    count
  end

  def part2
    dirs = @directions.cycle
    nodes = @adjacent.keys.find_all { |node| node.end_with?('A') }
    node_cycles = Array.new(nodes.length, nil)
    count = 0

    until nodes.length == node_cycles.compact.length
      dir = dirs.next
      nodes = nodes.map { |node| @adjacent[node][dir == 'L' ? 0 : 1] }
      count += 1

      nodes.each_with_index do |node, i|
        next if node_cycles[i]
        next unless node.end_with?('Z')

        node_cycles[i] = count
      end
    end

    node_cycles.reduce(1, :lcm)
  end
end

lines = ($stdin.read || '').split("\n")
Day08.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

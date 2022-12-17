# frozen_string_literal: true

require 'set'
require 'parallel'

Node = Struct.new(:name, :flow, :edges)
Edge = Struct.new(:to, :dist)
State = Struct.new(:current, :path, :time_left, :total) do
  def next_states(nodes)
    states = time_left.positive? ? [State.new(current, path, 0, nil)] : []

    current.edges.each do |edge|
      # don't move to existing open node
      next if path.include?(edge.to.name)
      next unless nodes.key?(edge.to.name)

      # time to move there + 1 for opening
      time_after_move = time_left - edge.dist - 1
      # don't move if we don't have enough time
      next if time_after_move.negative?

      states << State.new(edge.to, path + [edge.to.name], time_after_move, nil)
    end

    states.each do |next_state|
      next_state.total = total + path.sum { |name| nodes[name].flow } * (time_left - next_state.time_left)
    end
  end
end

class Day16
  def initialize(lines)
    @nodes = {}
    dist = Hash.new { |h, k| h[k] = Hash.new(Float::INFINITY) }

    lines.each do |line|
      first, second = line.split(';')
      name = first.split(' ')[1]
      flow = first.split('=').last.to_i
      node_neighbours = second.split(', ').map { |parts| parts.split(' ').last }

      @nodes[name] = Node.new(name, flow.to_i, [])

      dist[name][name] = 0
      node_neighbours.each do |neighbour|
        dist[name][neighbour] = 1
      end
    end

    # build edges
    @nodes.each_key do |k|
      @nodes.each_key do |i|
        @nodes.each_key do |j|
          dist[i][j] = [dist[i][j], dist[i][k] + dist[k][j]].min
        end
      end
    end

    @nodes.filter! { |_, node| node.name == 'AA' || node.flow.positive? }

    @nodes.values.combination(2).each do |(from, to)|
      from.edges << Edge.new(to, dist[from.name][to.name])
      to.edges << Edge.new(from, dist[to.name][from.name])
    end
  end

  def part1
    solve
  end

  def part2
    # is this cheating? probably
    Parallel.map(all_splits) do |(left, right)|
      solve(nodes: left, max_time: 26) + solve(nodes: right, max_time: 26)
    end.max
  end

  private

  def solve(nodes: @nodes, max_time: 30)
    queue = [State.new(nodes['AA'], ['AA'], max_time, 0)]
    max = 0

    until queue.empty?
      state = queue.shift
      state.next_states(nodes).each do |next_state|
        max = [next_state.total, max].max
        queue << next_state
      end
    end

    max
  end

  def all_splits
    keys = @nodes.keys - ['AA']
    (0..keys.length / 2).flat_map do |length|
      keys.combination(length).to_a.map do |left|
        right = keys - left
        [@nodes.slice('AA', *left), @nodes.slice('AA', *right)]
      end
    end
  end
end

lines = ($stdin.read || '').split("\n")
Day16.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

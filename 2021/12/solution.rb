# frozen_string_literal: true

require 'set'

class Path
  attr_reader :small_node_dup_count

  def initialize(start)
    @path = start
    s = small_nodes
    @small_node_dup_count = s.length - s.uniq.length
  end

  def add(node)
    @small_node_dup_count += 1 if node != :start && node.to_s.downcase == node.to_s && @path.include?(node)
    @path << node
  end

  def last
    @path.last
  end

  def seen?(node)
    @path.include?(node)
  end

  def ended?
    @path[-1] == :end
  end

  def dup
    Path.new(@path.dup)
  end

  def to_s
    @path.join(',')
  end

  private

  def small_nodes
    @path.filter { |node| node != :start && node.to_s.downcase == node.to_s }
  end
end

class Day12
  def initialize(lines)
    @adj = Hash.new { |h, k| h[k] = [] }

    lines.each do |line|
      start, finish = line.split('-').map(&:to_sym)
      @adj[start] << finish unless finish == :start
      @adj[finish] << start unless start == :start
    end
  end

  def part1
    solve(0)
  end

  def part2
    solve(1)
  end

  private

  def solve(small_path_visits)
    finished_paths = 0
    unfinished_paths = [Path.new([:start])]

    loop do
      path = unfinished_paths.shift
      break unless path

      if path.ended?
        finished_paths += 1
        next
      end

      @adj[path.last].each do |next_step|
        can_visit = !path.seen?(next_step) ||
          next_step.to_s == next_step.to_s.upcase ||
          path.small_node_dup_count < small_path_visits
        next unless can_visit

        new_path = path.dup
        new_path.add(next_step)
        unfinished_paths << new_path
      end
    end

    finished_paths
  end
end

lines = ($stdin.read || '').split("\n")
Day12.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

# frozen_string_literal: true

require 'set'

Ores = Struct.new(:ore, :clay, :obsidian, :geode) do
  def self.empty
    new(0, 0, 0, 0)
  end

  def self.from(ore: 0, clay: 0, obsidian: 0, geode: 0)
    new(ore, clay, obsidian, geode)
  end

  def +(other)
    Ores.new(ore + other.ore, clay + other.clay, obsidian + other.obsidian, geode + other.geode)
  end

  def -(other)
    Ores.new(ore - other.ore, clay - other.clay, obsidian - other.obsidian, geode - other.geode)
  end

  def <=(other)
    ore <= other.ore && clay <= other.clay && obsidian <= other.obsidian && geode <= other.geode
  end
end

Blueprint = Struct.new(:id, :ore_cost, :clay_cost, :obsidian_cost, :geode_cost) do
  # max ore needed per turn to buy any robot
  # don't need more than this many of each robot as we can't spend it fast enough
  def max_ores_needed
    @max_ores_needed ||= Ores.new(
      [ore_cost.ore, clay_cost.ore, obsidian_cost.ore, geode_cost.ore].max,
      [ore_cost.clay, clay_cost.clay, obsidian_cost.clay, geode_cost.clay].max,
      [ore_cost.obsidian, clay_cost.obsidian, obsidian_cost.obsidian, geode_cost.obsidian].max,
      Float::INFINITY
    )
  end
end

State = Struct.new(:available, :mining, :time) do
  def clamp_available_ores!(blueprint)
    # for this step and all future ones, we can only ever use at most Blueprint#max_ores_needed
    # clamp available ores to max needed ores for all remaining time steps minus ores which will be generated for each
    # remaining time step
    available.ore = [available.ore, (blueprint.max_ores_needed.ore * time) - mining.ore * (time - 1)].min
    available.clay = [available.clay, (blueprint.max_ores_needed.clay * time) - mining.clay * (time - 1)].min
    available.obsidian = [
      available.obsidian,
      (blueprint.max_ores_needed.obsidian * time) - mining.obsidian * (time - 1)
    ].min
  end
end

class Day19
  def initialize(lines)
    # normalise input
    lines = lines.join("\n").gsub("\n  ", ' ').gsub("\n\n", "\n").split("\n")

    @blueprints = lines.map.with_index do |line, index|
      # line = 'Blueprint 7: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 16 obsidian.'
      costs = line.split(': ').last.split('.').map { |cost| cost.split(' costs ').last }
      # costs = [
      #   '2 ore',
      #   '4 ore',
      #   '3 ore and 20 clay',
      #   '2 ore and 16 obsidian'
      # ]
      robot_costs = costs.map do |cost|
        # cost = '3 ore and 20 clay'
        robot_cost = Ores.empty
        cost.split(' and ').each do |cost_part|
          # cost_part = '20 clay'
          count, type = cost_part.split(' ')
          robot_cost[type] = count.to_i
        end
        robot_cost
      end

      Blueprint.new(index + 1, *robot_costs)
    end
  end

  def part1
    @blueprints.sum do |blueprint|
      blueprint.id * solve(blueprint, 24)
    end
  end

  def part2
    @blueprints.first(3).map do |blueprint|
      solve(blueprint, 32)
    end.reduce(:*)
  end

  private

  def solve(blueprint, time)
    best = 0
    best_idle = {}
    queue = [State.new(Ores.empty, Ores.from(ore: 1), time)]
    visited = Set.new

    while (state = queue.shift)
      if state.time.zero?
        best = [best, state.available.geode].max
        next
      end

      state.clamp_available_ores!(blueprint)
      next if visited.include?(state)

      visited << state

      if best_idle.key?(state.time)
        # assuming you can build a geode robot for all remaining times
        # can you beat the current best assuming they build no more robots
        # if not, then prune this branch
        max_possible = state.available.geode + (state.mining.geode..(state.mining.geode + state.time - 1)).sum
        next if max_possible <= best_idle[state.time]
      end
      curr_idle = state.available.geode + state.mining.geode * state.time
      best_idle[state.time] = [best_idle[state.time].to_i, curr_idle].max

      next_available = state.available + state.mining

      if blueprint.geode_cost <= state.available
        queue << State.new(
          next_available - blueprint.geode_cost,
          state.mining + Ores.from(geode: 1),
          state.time - 1
        )
        # assume that if you can build a geode miner, that's the optimal option
        next
      end

      if state.mining.obsidian < blueprint.max_ores_needed.obsidian && blueprint.obsidian_cost <= state.available
        queue << State.new(
          next_available - blueprint.obsidian_cost,
          state.mining + Ores.from(obsidian: 1),
          state.time - 1
        )
      end

      if state.mining.clay < blueprint.max_ores_needed.clay && blueprint.clay_cost <= state.available
        queue << State.new(
          next_available - blueprint.clay_cost,
          state.mining + Ores.from(clay: 1),
          state.time - 1
        )
      end

      if state.mining.ore < blueprint.max_ores_needed.ore && blueprint.ore_cost <= state.available
        queue << State.new(
          next_available - blueprint.ore_cost,
          state.mining + Ores.from(ore: 1),
          state.time - 1
        )
      end

      queue << State.new(next_available, state.mining, state.time - 1)
    end

    puts "Blueprint #{blueprint.id} best: #{best}"

    best
  end
end

lines = ($stdin.read || '').split("\n")
Day19.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

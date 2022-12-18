# frozen_string_literal: true

require 'set'

Point = Struct.new(:x, :y) do
  def delta(delta_x, delta_y)
    Point.new(x + delta_x, y + delta_y)
  end

  def left
    delta(-1, 0)
  end

  def right
    delta(1, 0)
  end

  def down
    delta(0, -1)
  end
end

World = Struct.new(:grid, :max_y) do
  def filled_set_for_seen
    set = Set.new

    # assume that the top 50 lines are enough to distinguish states
    # (i.e. nothing ever falls past the highest 50 lines)
    50.times.flat_map do |y|
      7.times.map do |x|
        set << Point.new(x, y) if grid[Point.new(x, max_y - y)]
      end
    end

    set
  end
end

SeenState = Struct.new(:rock, :wind, :grid_area)

class Rock
  attr_reader :shape, :position

  SHAPES = [
    [[true, true, true, true]],
    [
      [false, true, false],
      [true, true, true],
      [false, true, false]
    ],
    [
      [false, false, true],
      [false, false, true],
      [true, true, true]
    ],
    [[true], [true], [true], [true]],
    [[true, true], [true, true]]
  ].freeze

  def initialize(shape, world)
    @shape = shape
    # top left position
    @position = Point.new(2, shape.size + world.max_y + 3)
  end

  def move_down(world)
    return false if position.y == shape.size - 1

    new_location = position.down
    return false if collision?(world, new_location)

    @position = new_location
    true
  end

  def move_left(world)
    return false if position.x.zero?

    new_location = position.left
    return false if collision?(world, new_location)

    @position = new_location
    true
  end

  def move_right(world)
    return false if position.x + shape[0].size == 7

    new_location = position.right
    return false if collision?(world, new_location)

    @position = new_location
    true
  end

  def commit(world)
    shape.each_with_index do |row, y_offset|
      row.each_with_index do |filled, x_offset|
        next unless filled

        world.grid[position.delta(x_offset, -y_offset)] = true
      end
    end

    world.max_y = [world.max_y, position.y].max
  end

  private

  def collision?(world, new_location)
    shape.each_with_index do |row, y_offset|
      row.each_with_index do |filled, x_offset|
        next unless filled

        return true if world.grid[new_location.delta(x_offset, -y_offset)]
      end
    end

    false
  end
end

class Day17
  def initialize(line)
    @line = line
  end

  def part1
    solve(2022)
  end

  def part2
    solve(1_000_000_000_000)
  end

  private

  def solve(rock_limit)
    seen_states = {}
    height_from_cycles = 0

    gas_gen = @line.chars.cycle
    rock_shape_gen = Rock::SHAPES.cycle
    world = World.new(Hash.new(false), -1)
    rock = Rock.new(rock_shape_gen.next, world)
    rocks_dropped = 0

    loop do
      gas_gen.next == '<' ? rock.move_left(world) : rock.move_right(world)
      next if rock.move_down(world)

      rock.commit(world)
      rocks_dropped += 1
      rock = Rock.new(rock_shape_gen.next, world)
      break if rocks_dropped == rock_limit

      # cycle detection for part 2
      state = SeenState.new(rock_shape_gen.peek, gas_gen.peek, world.filled_set_for_seen)
      if seen_states.key?(state)
        cycle_rocks = rocks_dropped - seen_states[state][:rocks]
        cycle_height = world.max_y - seen_states[state][:height]
        cycle_runs = (rock_limit - rocks_dropped) / cycle_rocks

        if cycle_runs.positive?
          rocks_dropped += cycle_runs * cycle_rocks
          height_from_cycles += cycle_runs * cycle_height
        end
      else
        seen_states[state] = { rocks: rocks_dropped, height: world.max_y }
      end
    end

    world.max_y + 1 + height_from_cycles
  end
end

line = ($stdin.read || '').split("\n")[0]
Day17.new(line).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

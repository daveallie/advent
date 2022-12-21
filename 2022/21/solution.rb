# frozen_string_literal: true

Num = Struct.new(:value) do
  def calc(_map)
    value
  end

  def clear_memo
    # noop
  end
end

Op = Struct.new(:left, :op, :right) do
  def calc(map)
    @calc ||= map[left].calc(map).send(op, map[right].calc(map))
  end

  def clear_memo
    @calc = nil
  end
end

class Day21
  def initialize(lines)
    @lines = lines
    @map = @lines.to_h do |line|
      key, val = line.split(': ')
      left, op, right = val.split(' ')

      [key.to_sym, op ? Op.new(left.to_sym, op.to_sym, right.to_sym) : Num.new(left.to_i)]
    end
  end

  def part1
    @map.each_value(&:clear_memo)

    @map[:root].calc(@map)
  end

  def part2
    @map.each_value(&:clear_memo)

    path_to_humn = build_path_to_humn[1..]
    target = @map[path_to_humn[0] == @map[:root].left ? @map[:root].right : @map[:root].left].calc(@map)
    curr = path_to_humn.shift

    # reverse each equation in the path to get to humn
    while curr != :humn
      node = @map[curr]
      left_node = @map[node.left]
      right_node = @map[node.right]
      op = node.op
      humn_on_left = path_to_humn[0] == node.left

      target = case op
               when :+
                 if humn_on_left
                   # left = target - right
                   target - right_node.calc(@map)
                 else
                   # right = target - left
                   target - left_node.calc(@map)
                 end
               when :-
                 if humn_on_left
                   # left = target + right
                   target + right_node.calc(@map)
                 else
                   # right = left - target
                   left_node.calc(@map) - target
                 end
               when :*
                 if humn_on_left
                   # left = target / right
                   target / right_node.calc(@map)
                 else
                   # right = target / left
                   target / left_node.calc(@map)
                 end
               when :/
                 if humn_on_left
                   # left = target * right
                   target * right_node.calc(@map)
                 else
                   # right = left / target
                   left_node.calc(@map) / target
                 end
               else
                 raise "unknown op #{op}"
               end
      curr = path_to_humn.shift
    end

    target
  end

  private

  # return a path of monkeys from root to humn
  def build_path_to_humn
    old_humn = @map[:humn]
    @map[:humn] = nil

    path_to_humn = [:root]
    loop do
      curr = @map[path_to_humn.last]
      left_node = @map[curr.left]
      right_node = @map[curr.right]

      if left_node.nil? || right_node.nil?
        path_to_humn << :humn
        break
      end

      # will throw if left is not calcuable
      # using exceptions for control flow is bad, m'kay
      begin
        left_node.calc(@map)
        path_to_humn << curr.right
      rescue StandardError
        path_to_humn << curr.left
      end
    end

    @map[:humn] = old_humn
    path_to_humn
  end
end

lines = ($stdin.read || '').split("\n")
Day21.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

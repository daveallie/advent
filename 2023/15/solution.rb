# frozen_string_literal: true

class Day15
  def initialize(input)
    @parts = input.strip.split(',')
  end

  def part1
    @parts.sum { |p| calc_hash(p) }
  end

  def part2
    boxes = {}

    @parts.each do |p|
      label = p.split('-').first
      label, focal = label.split('=')

      box = calc_hash(label)
      boxes[box] ||= []

      existing_index = boxes[box].find_index { |b| b[0] == label }

      if focal
        if existing_index
          boxes[box][existing_index][1] = focal.to_i
        else
          boxes[box] << [label, focal.to_i]
        end
      elsif existing_index
        boxes[box].delete_at(existing_index)
      end
    end

    boxes.sum { |k, lenses| lenses.each_with_index.sum { |l, i| l[1] * (i + 1) } * (k + 1) }
  end

  private

  def calc_hash(str)
    str.chars.map(&:ord).reverse.each_with_index.sum { |c, i| c * 17.pow(i + 1) } % 256
  end
end

input = ($stdin.read || '')
Day15.new(input).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

# ((((a1 * 17) % 256) + a2) * 17) % 256

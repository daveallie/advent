# frozen_string_literal: true

class Card
  def initialize(winning_nums, found_nums)
    @winning_nums = winning_nums
    @found_nums = found_nums
  end

  def count_matching_nums
    (@winning_nums & @found_nums).count
  end
end

class Day04
  def initialize(lines)
    @cards = {}

    lines.each do |line|
      card_details, all_nums = line.split(':')

      id = card_details.split(' ').last.to_i
      winning_nums, found_nums = all_nums.strip
                                         .split(' | ')
                                         .map { |nums| nums.split(/\s+/).map(&:to_i) }

      @cards[id] = Card.new(winning_nums, found_nums)
    end
  end

  def part1
    @cards.values.sum do |c|
      match_count = c.count_matching_nums
      match_count.positive? ? 2.pow(match_count - 1) : 0
    end
  end

  def part2
    cards_gend = {}

    @cards.keys.sum do |id|
      p2_solve(id, cards_gend)
    end
  end

  private

  def p2_solve(id, cards_gend)
    return cards_gend[id] if cards_gend[id]

    card = @cards[id]
    match_count = card.count_matching_nums
    cards_gend[id] = 1

    if match_count.positive?
      new_ids = ((id + 1)..(id + match_count)).to_a
      cards_gend[id] += new_ids.sum { |new_id| p2_solve(new_id, cards_gend) }
    end

    cards_gend[id]
  end
end

lines = ($stdin.read || '').split("\n")
Day04.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

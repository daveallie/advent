# frozen_string_literal: true

OPPOSITE = {
  up: :down,
  down: :up,
  left: :right,
  right: :left
}.freeze

ROTATION = {
  up: :right,
  right: :down,
  down: :left,
  left: :up
}.freeze

DIR_VAL = {
  right: 0,
  down: 1,
  left: 2,
  up: 3
}.freeze

def build_cube_net_transitions
  output = {
    # all possible moves to get from one tile to the tile on its right, and that tile's joining edge
    # the rest of the directions can be rotated from this
    right: [
      # dist 1
      [%i[right], :left],

      # dist 2
      [%i[down right], :up],
      [%i[up right], :down],

      # dist 3
      [%i[left left left], :left],
      [%i[down down right], :right],
      [%i[up up right], :right],
      [%i[left up up], :right],
      [%i[left down down], :right],

      # dist 4
      [%i[left left down left], :down],
      [%i[left left up left], :up],
      [%i[left down left down], :up],
      [%i[left up left up], :up],
      [%i[down left left left], :up],
      [%i[up left left left], :down],
      [%i[down left down down], :down],
      [%i[up left up up], :up],
      [%i[down down down right], :down],
      [%i[up up up right], :up],

      # dist 5
      [%i[left down left left down], :left],
      [%i[left up left left up], :left],
      [%i[down left left down left], :left],
      [%i[up left left up left], :left],
      [%i[down left down left down], :left],
      [%i[up left up left up], :left],
      [%i[down down left down down], :left],
      [%i[up up left up up], :left]
    ]
  }

  source = :right
  3.times do
    output[ROTATION[source]] = output[source].map do |(moves, edge)|
      [moves.map { |move| ROTATION[move] }, ROTATION[edge]]
    end
    source = ROTATION[source]
  end

  output
end

CUBE_NET_TRANSITIONS = build_cube_net_transitions.freeze

Point = Struct.new(:x, :y) do
  def move(dir)
    case dir
    when :up
      Point.new(x, y - 1)
    when :down
      Point.new(x, y + 1)
    when :left
      Point.new(x - 1, y)
    when :right
      Point.new(x + 1, y)
    end
  end
end

Node = Struct.new(:point, :is_wall, :face)

DistMove = Struct.new(:amount)

DirMove = Struct.new(:left) do
  def next_dir(curr_dir)
    next_dir = ROTATION[curr_dir]
    left ? OPPOSITE[next_dir] : next_dir
  end
end


Face = Struct.new(:coords, :side_length, :neighbours) do
  def self.to_face_coords(point, side_length)
    Point.new((point.x - 1) / side_length, (point.y - 1) / side_length)
  end

  def build_neighbours(face_map, part2: false)
    set_neighbour(:up, *find_neighbour(face_map, :up, part2: part2))
    set_neighbour(:down, *find_neighbour(face_map, :down, part2: part2))
    set_neighbour(:left, *find_neighbour(face_map, :left, part2: part2))
    set_neighbour(:right, *find_neighbour(face_map, :right, part2: part2))
  end

  def set_neighbour(edge, face, target_edge)
    neighbours[edge] = { face: face, target_edge: target_edge }
  end

  def move(point, dir)
    local_point = to_local(point)

    # move within face
    if dir == :up && local_point.y > 1 ||
       dir == :down && local_point.y < side_length ||
       dir == :left && local_point.x > 1 ||
       dir == :right && local_point.x < side_length
      return [point.move(dir), dir]
    end

    neighbour = neighbours[dir]
    target_edge = neighbour[:target_edge]
    next_local_point =
      case [dir, target_edge]
      # normal transition
      when %i[up down], %i[down up]
        Point.new(local_point.x, target_edge == :up ? 1 : side_length)
      when %i[left right], %i[right left]
        Point.new(target_edge == :left ? 1 : side_length, local_point.y)

      # transiting to mirrored face
      when %i[up up], %i[down down]
        Point.new(side_length - local_point.x + 1, target_edge == :up ? 1 : side_length)
      when %i[left left], %i[right right]
        Point.new(target_edge == :left ? 1 : side_length, side_length - local_point.y + 1)

      # right angled transitions
      when %i[up left], %i[down right]
        Point.new(target_edge == :left ? 1 : side_length, local_point.x)
      when %i[left up], %i[right down]
        Point.new(local_point.y, target_edge == :up ? 1 : side_length)
      when %i[up right], %i[down left]
        Point.new(target_edge == :left ? 1 : side_length, side_length - local_point.x + 1)
      when %i[right up], %i[left down]
        Point.new(side_length - local_point.y + 1, target_edge == :up ? 1 : side_length)
      else
        raise "unhandled transition: #{dir} => #{target_edge}"
      end

    [neighbour[:face].to_global(next_local_point), OPPOSITE[target_edge]]
  end

  # convert from point within face to global point
  def to_global(local_point)
    Point.new(local_point.x + side_length * coords.x, local_point.y + side_length * coords.y)
  end

  # convert from global point to point within face
  def to_local(global_point)
    Point.new(global_point.x - side_length * coords.x, global_point.y - side_length * coords.y)
  end

  private

  def find_neighbour(face_map, dir, part2: false)
    if part2
      find_neighbour_part2(face_map, dir)
    else
      find_neighbour_part1(face_map, dir)
    end
  end

  # all neighbours are either adjacent or direct wrap arounds
  def find_neighbour_part1(face_map, dir)
    return [face_map[coords.move(dir)], OPPOSITE[dir]] if face_map.key?(coords.move(dir))

    adj_coords =
      case dir
      when :up
        face_map.keys.filter { |point| point.x == coords.x }.max_by(&:y)
      when :down
        face_map.keys.filter { |point| point.x == coords.x }.min_by(&:y)
      when :left
        face_map.keys.filter { |point| point.y == coords.y }.max_by(&:x)
      when :right
        face_map.keys.filter { |point| point.y == coords.y }.min_by(&:x)
      end

    [face_map[adj_coords], OPPOSITE[dir]]
  end

  # faces form a cube net, use const to face and target edge
  def find_neighbour_part2(face_map, dir)
    CUBE_NET_TRANSITIONS[dir].lazy.map do |moves, target_edge|
      point = coords
      failed = false
      moves.each do |move|
        # try to move from current face to neighbour through moves
        # fail fast if any move is invalid
        point = point.move(move)
        unless face_map.key?(point)
          failed = true
          break
        end
      end
      next if failed

      [face_map[point], target_edge]
    end.reject(&:nil?).first
  end
end

class Day22
  def initialize(lines)
    @map, moves = lines.slice_after('').to_a
    moves = moves[0]

    chars = @map.map(&:chars)
    min_width = chars.filter { |row| row.length.positive? }.map { |row| row.count { |char| char != ' ' } }.min
    min_height = (0...chars.map(&:length).max).map do |col|
      chars.count { |row| row[col] && row[col] != ' ' }
    end.min
    @side_length = [min_width, min_height].min

    @moves = []
    while moves.length.positive?
      move_dist, turn_dir, moves = moves.partition(/[LR]/)
      @moves << DistMove.new(move_dist.to_i) if move_dist.length.positive?
      @moves << DirMove.new(turn_dir == 'L') if turn_dir.length.positive?
    end
  end

  def part1
    face_map = build_face_map
    solve(build_grid(face_map))
  end

  def part2
    face_map = build_face_map(part2: true)
    solve(build_grid(face_map))
  end

  private

  def solve(grid)
    curr = grid.values
               .sort_by { |node| [node.point.y, node.point.x] }
               .find { |node| !node.is_wall }
               .point
    dir = :right

    @moves.each do |move|
      if move.is_a?(DirMove)
        dir = move.next_dir(dir)
        next
      end

      move.amount.times do
        next_node, next_dir = grid[curr].face.move(curr, dir)
        break if grid[next_node].is_wall

        curr = next_node
        dir = next_dir
      end
    end

    output(curr, dir)
  end

  def output(point, dir)
    point.y * 1000 + point.x * 4 + DIR_VAL[dir]
  end

  def build_face_map(part2: false)
    faces = {}
    # could exist on 4x4 grid
    chars = @map.map(&:chars)
    4.times do |y|
      4.times do |x|
        next unless chars[@side_length * y] && %w[. #].include?(chars[@side_length * y][@side_length * x])

        point = Point.new(x, y)
        faces[point] = Face.new(point, @side_length, {})
      end
    end

    faces.each_value { |face| face.build_neighbours(faces, part2: part2) }
    faces
  end

  def build_grid(face_map)
    grid = {}

    @map.each_with_index do |row, y|
      row.chars.each_with_index do |cell, x|
        next if cell == ' '

        point = Point.new(x + 1, y + 1)
        face = face_map[Face.to_face_coords(point, @side_length)]
        grid[point] = Node.new(point, cell == '#', face)
      end
    end

    grid
  end
end

lines = ($stdin.read || '').split("\n")
Day22.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

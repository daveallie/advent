# frozen_string_literal: true

Point = Struct.new(:x, :y)

DistMove = Struct.new(:amount)

DirMove = Struct.new(:left) do
  def next_dir(curr_dir)
    case curr_dir
    when :up
      left ? :left : :right
    when :down
      left ? :right : :left
    when :left
      left ? :down : :up
    when :right
      left ? :up : :down
    else
      raise 'unknown dir'
    end
  end
end

Face = Struct.new(:coords, :side_length, :neighbours) do
  def set_neighbour(edge, face, target_edge)
    neighbours[edge] = { face: face, target_edge: target_edge }
  end

  def move(point, dir)
    local_point = to_local(point)

    # move within face
    return [Point.new(point.x, point.y - 1), dir] if dir == :up && local_point.y > 1
    return [Point.new(point.x, point.y + 1), dir] if dir == :down && local_point.y < side_length
    return [Point.new(point.x - 1, point.y), dir] if dir == :left && local_point.x > 1
    return [Point.new(point.x + 1, point.y), dir] if dir == :right && local_point.x < side_length

    neighbour = neighbours[dir]
    next_local_point =
      case [dir, neighbour[:target_edge]]
      # normal transition
      when %i[up down]
        Point.new(local_point.x, side_length)
      when %i[down up]
        Point.new(local_point.x, 1)
      when %i[left right]
        Point.new(side_length, local_point.y)
      when %i[right left]
        Point.new(1, local_point.y)

      # transiting to mirrored face
      when %i[up up]
        Point.new(side_length - local_point.x + 1, 1)
      when %i[down down]
        Point.new(side_length - local_point.x + 1, side_length)
      when %i[left left]
        Point.new(1, side_length - local_point.y + 1)
      when %i[right right]
        Point.new(side_length, side_length - local_point.y + 1)

      # right angled transitions
      when %i[up left]
        Point.new(1, local_point.x)
      when %i[left up]
        Point.new(local_point.y, 1)
      when %i[down right]
        Point.new(side_length, local_point.x)
      when %i[right down]
        Point.new(local_point.y, side_length)
      when %i[up right]
        Point.new(side_length, side_length - local_point.x + 1)
      when %i[right up]
        Point.new(side_length - local_point.y + 1, 1)
      when %i[down left]
        Point.new(1, side_length - local_point.x + 1)
      when %i[left down]
        Point.new(side_length - local_point.y + 1, side_length)
      else
        raise "unhandled transition: #{dir} => #{neighbour[:target_edge]}"
      end

    next_dir =
      case neighbour[:target_edge]
      when :up
        :down
      when :down
        :up
      when :left
        :right
      when :right
        :left
      else
        raise "unknown dir #{dir}"
      end

    [neighbour[:face].to_global(next_local_point), next_dir]
  end

  def to_global(local_point)
    Point.new(local_point.x + side_length * coords.x, local_point.y + side_length * coords.y)
  end

  def to_local(global_point)
    Point.new(global_point.x - side_length * coords.x, global_point.y - side_length * coords.y)
  end
end

Node = Struct.new(:point, :is_wall, :face)

# would be cool to have this be generated from input, but at this point, I cbf
class FaceMap
  class << self
    def build_sample(part2: false)
      # sample(side=4)
      # - - 0
      # 1 2 3
      # - - 4 5

      edge_joins =
        if part2
          # cube layout
          [
            [[0, :up], [1, :up]],
            [[0, :down], [3, :up]],
            [[0, :left], [2, :up]],
            [[0, :right], [5, :right]],
            [[1, :down], [4, :down]],
            [[1, :left], [5, :down]],
            [[1, :right], [2, :left]],
            [[2, :down], [4, :left]],
            [[2, :right], [3, :left]],
            [[3, :down], [4, :up]],
            [[3, :right], [5, :up]],
            [[4, :right], [5, :left]]
          ]
        else
          # flat layout
          [
            [[0, :up], [4, :down]],
            [[0, :down], [3, :up]],
            [[0, :left], [0, :right]],
            [[1, :up], [1, :down]],
            [[1, :left], [3, :right]],
            [[1, :right], [2, :left]],
            [[2, :up], [2, :down]],
            [[2, :right], [3, :left]],
            [[3, :down], [4, :up]],
            [[4, :left], [5, :right]],
            [[4, :right], [5, :left]],
            [[5, :up], [5, :down]]
          ]
        end

      build_face_map(
        [
          Point.new(2, 0),
          Point.new(0, 1),
          Point.new(1, 1),
          Point.new(2, 1),
          Point.new(2, 2),
          Point.new(3, 2)
        ],
        edge_joins,
        4
      )
    end

    def build_actual(part2: false)
      # real(side=50)
      # - 0 1
      # - 2
      # 3 4
      # 5

      edge_joins =
        if part2
          # cube layout
          [
            [[0, :up], [5, :left]],
            [[0, :down], [2, :up]],
            [[0, :left], [3, :left]],
            [[0, :right], [1, :left]],
            [[1, :up], [5, :down]],
            [[1, :down], [2, :right]],
            [[1, :right], [4, :right]],
            [[2, :down], [4, :up]],
            [[2, :left], [3, :up]],
            [[3, :down], [5, :up]],
            [[3, :right], [4, :left]],
            [[4, :down], [5, :right]]
          ]
        else
          # flat layout
          [
            [[0, :up], [4, :down]],
            [[0, :down], [2, :up]],
            [[0, :left], [1, :right]],
            [[0, :right], [1, :left]],
            [[1, :up], [1, :down]],
            [[2, :down], [4, :up]],
            [[2, :left], [2, :right]],
            [[3, :up], [5, :down]],
            [[3, :down], [5, :up]],
            [[3, :left], [4, :right]],
            [[3, :right], [4, :left]],
            [[5, :left], [5, :right]]
          ]
        end

      build_face_map(
        [
          Point.new(1, 0),
          Point.new(2, 0),
          Point.new(1, 1),
          Point.new(0, 2),
          Point.new(1, 2),
          Point.new(0, 3)
        ],
        edge_joins,
        50
      )
    end

    private


    def build_face_map(coords, joins, side_length)
      faces = coords.map do |coord|
        Face.new(coord, side_length, {})
      end

      joins.each do |(from_face_i, from_edge), (to_face_i, to_edge)|
        from_face = faces[from_face_i]
        to_face = faces[to_face_i]
        from_face.set_neighbour(from_edge, to_face, to_edge)
        to_face.set_neighbour(to_edge, from_face, from_edge)
      end

      faces.to_h { |face| [face.coords, face] }
    end
  end
end

class Day22
  DIR_VAL = {
    right: 0,
    down: 1,
    left: 2,
    up: 3
  }.freeze

  def initialize(lines)
    @map, moves = lines.slice_after('').to_a
    moves = moves[0]

    @moves = []
    while moves.length.positive?
      move_dist, turn_dir, moves = moves.partition(/[LR]/)
      @moves << DistMove.new(move_dist.to_i) if move_dist.length.positive?
      @moves << DirMove.new(turn_dir == 'L') if turn_dir.length.positive?
    end
  end

  def part1
    # faces = FaceMap.build_sample
    faces = FaceMap.build_actual
    solve(parse_map(faces))
  end

  def part2
    # faces = FaceMap.build_sample(part2: true)
    faces = FaceMap.build_actual(part2: true)
    solve(parse_map(faces))
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

  def parse_map(face_map)
    grid = {}
    side_length = face_map.values.first.side_length
    @map.each_with_index do |row, y|
      row.chars.each_with_index do |cell, x|
        next if cell == ' '

        point = Point.new(x + 1, y + 1)
        grid[point] = Node.new(point, cell == '#', face_map[Point.new(x / side_length, y / side_length)] {})
      end
    end
    grid
  end

  def output(point, dir)
    point.y * 1000 + point.x * 4 + DIR_VAL[dir]
  end
end

lines = ($stdin.read || '').split("\n")
Day22.new(lines).tap do |day|
  puts "Part 1: #{day.part1}"
  puts "Part 2: #{day.part2}"
end

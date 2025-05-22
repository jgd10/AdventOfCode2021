from aoc import parse_file, Point, InputType
from dataclasses import dataclass


@dataclass(frozen=True)
class Line:
    point1: Point
    point2: Point

    def get_points_in_line(self):
        diff = self.point2.diff(self.point1)
        unit_diff = diff.div_mod(diff.length)
        points = {self.point1, self.point2}
        p = self.point1
        for i in range(diff.length):
            p = p.add(unit_diff)
            points.add(p)
        return points


@dataclass
class Grid:
    lines: list[Line]

    def fill_points(self):
        num = 0
        points = {}
        for line in self.lines:
            if line.point1.x != line.point2.x or line.point1.y != line.point2.y:
                continue
            for p in line.get_points_in_line():
                if p in points:
                    points[p] += 1
                else:
                    points[p] = 1
        return points

    def get_overlaps(self):
        points = self.fill_points()
        return len({p for p in points.values() if p>1})


def parse(input_type: InputType):
    data = parse_file(input_type)
    lines = []
    for row in data:
        start, end = row.split(' -> ')
        points = []
        for p in [start, end]:
            vals = [int(s) for s in p.split(',')]
            points.append(Point(*vals))
        line = Line(points[0], points[1])
        lines.append(line)
    return Grid(lines)


def part1(input_type: InputType):
    grid = parse(input_type)
    print(f'Part 1: {grid.get_overlaps()}')


if __name__ == '__main__':
    part1(InputType.EXAMPLE)

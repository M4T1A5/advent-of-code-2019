use std::fs::File;
use std::io::prelude::*;
use std::f32;

enum LineDirection
{
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Vec2
{
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct LineSegment
{
    start: (i32, i32),
    end: (i32, i32),
}

impl LineSegment {
    pub fn new() -> LineSegment {
        LineSegment { start: (0, 0), end: (0, 0) }
    }

    pub fn from_to(start: (i32, i32), end: (i32, i32)) -> LineSegment {
        LineSegment { start, end }
    }

    pub fn offset_from_line(start: LineSegment, offset: (i32, i32)) -> LineSegment {
        LineSegment { start: start.end, end: (start.end.0 + offset.0, start.end.1 + offset.1) }
    }
}

#[derive(Debug)]
struct Line
{
    segments: Vec<LineSegment>
}

impl Line {
    pub fn new() -> Line {
        Line { segments: vec![] }
    }
}

fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input_text = String::new();
    input_file.read_to_string(&mut input_text).unwrap();

    let line_strings: Vec<&str> = input_text.trim().split("\n").collect();
    let mut lines = vec![];
    for line_string in line_strings {
        let mut line = Line::new();
        for line_segment in line_string.split(",") {
            let line_segment = line_segment.trim() as &str;
            let line_direction = match line_segment.chars().nth(0).unwrap() {
                'U' => LineDirection::Up(line_segment[1..].parse().unwrap()),
                'D' => LineDirection::Down(line_segment[1..].parse().unwrap()),
                'L' => LineDirection::Left(line_segment[1..].parse().unwrap()),
                'R' => LineDirection::Right(line_segment[1..].parse().unwrap()),
                _ => panic!("Unrecognized line segment")
            };

            let previous_segment = if let Some(x) = line.segments.iter().last() {
                x.clone()
            } else {
                LineSegment::new()
            };
            line.segments.push(match line_direction {
                LineDirection::Up(x) => LineSegment::offset_from_line(previous_segment, (0, x)),
                LineDirection::Down(x) => LineSegment::offset_from_line(previous_segment, (0, -x)),
                LineDirection::Left(x) => LineSegment::offset_from_line(previous_segment, (-x, 0)),
                LineDirection::Right(x) => LineSegment::offset_from_line(previous_segment, (x, 0)),
            });
        }
        lines.push(line);
    }

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].segments.len(), lines[1].segments.len());

    let line1 = &lines[0];
    let line2 = &lines[1];

    // Original problem
    let mut intersection_points = vec![];
    for line1_segment in &line1.segments {
        for line2_segment in &line2.segments {
            // check for intersection (https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line)
            let x1 = line1_segment.start.0;
            let y1 = line1_segment.start.1;
            let x2 = line1_segment.end.0;
            let y2 = line1_segment.end.1;
            let x3 = line2_segment.start.0;
            let y3 = line2_segment.start.1;
            let x4 = line2_segment.end.0;
            let y4 = line2_segment.end.1;
            // (x1-x2)(y3-y4) - (y1-y2)(x3-x4)
            let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

            // Intersection impossible, skip
            if denominator == 0 { continue; };

            // (x1-x3)(y3-y4) - (y1-y3)(x3-x4) / denominator
            let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f32 / denominator as f32;
            // (x1-x2)(y1-y3) - (y1-y2)(x1-x2) / denominator
            let u = ((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x2)) as f32 / denominator as f32;
            if t >= 0. && t <= 1. {
                intersection_points.push((x1 as f32 + t * (x2 - x1) as f32,
                                          y1 as f32 + t * (y2 - y1) as f32));
            } else if u >= 0. && u <= 1. {
                intersection_points.push((x3 as f32 + u * (x4 - x3) as f32,
                                          y3 as f32 + u * (y4 - y3) as f32));
            }
        }
    }

    let mut closest_intersection = f32::MAX;
    for intersection_point in intersection_points {
        let manhattan_distance = intersection_point.0 + intersection_point.1;
        if manhattan_distance.abs() < closest_intersection {
            closest_intersection = manhattan_distance;
        }
    }
    println!("Closest intersection {}", closest_intersection);
}

use regex::Regex;
use std::collections::HashSet;
use std::cmp::max;


#[derive(Clone,Debug)]
enum Error {
    UnexpectedParsingError,
    PatternMismatch,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Area {
    min: Point,
    max: Point,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: isize::default(), y: isize::default() }
    }
}

struct ArcIteratorY<'a> {
    area: &'a Area,
    time: isize,
    y: isize,
    time_max: isize,
}

#[derive(Debug, Clone, Copy)]
struct Shot {
    velocity: isize,
    time: isize,
}

impl<'a> ArcIteratorY<'a> {
    fn new(area: &'a Area) -> Self {
        ArcIteratorY {
            area,
            time: 1,
            y: area.min.y,
            time_max: 2 * max(area.min.y.abs(), area.max.y.abs())
        }
    }
}

struct ArcIteratorX<'a> {
    area: &'a Area,
    time: isize,
    dx: Option<isize>,
}

impl<'a> ArcIteratorX<'a> {
    fn stop(&self, d: isize) -> isize {
        let m = self.time;
        if d <= m { (d * (d + 1)) / 2 } else { m * d - ((m - 1) * m) / 2 }
    }

    fn new(area: &'a Area, time: isize) -> Self {
        ArcIteratorX { area, time, dx: Some(0) }
    }
}

impl<'a> Iterator for ArcIteratorY<'a> {
    type Item = Shot;

    fn next(&mut self) -> Option<Self::Item> {
        for t in self.time..=self.time_max {
            for y in self.y..=self.area.max.y {
                if 2 * y % t != 0 {
                    continue;
                }
                let dy = (2 * y / t) + t - 1;
                if dy % 2 != 0 {
                    continue;
                }
                self.y = y + 1;
                self.time = t;
                return Some(Shot{velocity: dy / 2, time: t});
            }
            self.y = self.area.min.y;
            self.time = t + 1;
        }
        None
    }
}

impl<'a> Iterator for ArcIteratorX<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(dx) = self.dx {
            if dx <= 0 {
                let dx = -dx;
                for k in dx.. {
                    let stop = -self.stop(k);
                    if stop < self.area.min.x {
                        if self.area.max.x > 0 {
                            self.dx = Some(1);
                            return self.next();
                        }
                        self.dx = None;
                        break;
                    } else if stop <= self.area.max.x {
                        self.dx = Some(-k - 1);
                        return Some(-k);
                    }
                }
            } else {
                for k in dx.. {
                    let stop = self.stop(k);
                    if stop > self.area.max.x {
                        self.dx = None;
                        break;
                    } else if stop >= self.area.min.x {
                        self.dx = Some(k + 1);
                        return Some(k);
                    }
                }
            }
        }
        self.dx
    }

}

impl Area {
    fn new(spec: &str) -> Result<Self, Error> {
        let pattern = Regex::new(
            r"x=(-?\d+)\.\.(-?\d+),\s*y=(-?\d+)\.\.(-?\d+)").unwrap();
        if let Some(captures) = pattern.captures(spec) {
            let captures: Option<Vec<_>> = captures.iter().skip(1).collect();
            let values = captures.ok_or(Error::UnexpectedParsingError)?;
            let captures: Result<Vec<isize>, _> = values
                .iter().map(|&m| isize::from_str_radix(m.as_str(), 10)).collect();
            let captures = captures.map_err(|_| Error::UnexpectedParsingError)?;
            Ok(Area{
                min: Point { x: captures[0], y: captures[2] },
                max: Point { x: captures[1], y: captures[3] },
            })
        } else {
            Err(Error::PatternMismatch)
        }
    }

    fn count_possible_shots(&self) -> usize {
        let velocities: HashSet<(isize, isize)> = ArcIteratorY::new(self)
            .flat_map(|a| ArcIteratorX::new(self, a.time).map(move |x| (x, a.velocity)))
            .collect();
        velocities.len()
    }

    fn highest_altitude(&self) -> isize {
        let mut best_apex = 0;
        for shot in ArcIteratorY::new(self) {
            let d = shot.velocity;
            let m = shot.time;
            if ArcIteratorX::new(self, m).next().is_none() {
                continue;
            }
            let apex = if d <= m { (d * (d + 1)) / 2 } else { m * d - ((m - 1) * m) / 2 };
            if apex > best_apex {
                best_apex = apex;
            }
        }
        best_apex
    }

}

fn main() {
    if let Ok(area) = Area::new("target area: x=20..30, y=-10..-5") {
        println!("test/part1: {:?}", area.highest_altitude());
        println!("test/part2: {:?}", area.count_possible_shots());
    }
    if let Ok(area) = Area::new("target area: x=56..76, y=-162..-134") {
        println!("main/part1: {:?}", area.highest_altitude());
        println!("main/part2: {:?}", area.count_possible_shots());
    }
}

use piston::Position;

pub struct Line {
  to: Position,
  from: Position
}

impl Line {
  pub fn len(&self) -> f64 {
    let x_delta = (self.from.x - self.to.x).abs();
    let y_delta = (self.from.y - self.to.y).abs();
    ((x_delta.pow(2) + y_delta.pow(2)) as f64).sqrt()
  }

  fn on_segment(&self, p: Position, q: Position, r: Position) -> bool {
    if (q.x <= std::cmp::max(p.x, r.x)) && (q.x >= std::cmp::min(p.x, r.x)) &&
       (q.y <= std::cmp::max(p.y, r.y)) && (q.y >= std::cmp::min(p.y, r.y)) {
      return true;
    }

    false
  }

  fn orientation(&self, p: Position, q: Position, r: Position) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val == 0 {
      0
    } else if val > 0 {
      1
    } else {
      2
    }
  }

  pub fn intersects(&self, line: &Line) -> bool {
    let o1 = self.orientation(self.from, self.to, line.from);
    let o2 = self.orientation(self.from, self.to, line.to);
    let o3 = self.orientation(line.from, line.to, self.from);
    let o4 = self.orientation(line.from, line.to, self.to);

    if o1 != o2 && o3 != o4 {
      return true;
    }

    if o1 == 0 && self.on_segment(self.from, line.to, self.to) {
      return true;
    }

    if o2 == 0 && self.on_segment(self.from, line.to, self.to) {
      return true;
    }

    if o3 == 0 && self.on_segment(line.from, self.from, line.to) {
      return true;
    }

    if o4 == 0 && self.on_segment(line.from, self.to, line.to) {
      return true;
    }

    false
  }

  pub fn get_from(self) -> Position {
    self.from
  }

  pub fn get_to(self) -> Position {
    self.to
  }

  pub fn set_to(&mut self, to: Position) {
    self.to = to;
  }

  pub fn list_state(&self) {
    println!("  {},{} {},{}", self.from.x, self.from.y, self.to.x, self.to.y);
  }
}

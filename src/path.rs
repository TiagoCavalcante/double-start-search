use graphs::Graph;
use std::collections::VecDeque;

// Returns whether number `a` is closer to `to` than `b`.
fn closer_to(a: usize, b: usize, to: usize) -> bool {
  match (a > to, b > to) {
    (true, true) => a - to < b - to,
    (true, false) => a - to < to - b,
    (false, true) => to - a < b - to,
    (false, false) => to - a < to - b,
  }
}

pub fn double_start_search(
  graph: &Graph,
  start: usize,
  end: usize,
  length: usize,
) -> Vec<usize> {
  // A queue to maintain the vertices whose adjacency list
  // is to be scanned as per normal DFS algorithm.
  let mut queue = VecDeque::new();

  let mut predecessor_from_start =
    vec![usize::MAX; graph.size];
  let mut predecessor_from_end =
    vec![usize::MAX; graph.size];
  let mut distance_to_start = vec![usize::MAX; graph.size];
  let mut distance_to_end = vec![usize::MAX; graph.size];

  // The distance to the start is 0, but we set it to 1 so
  // we don't need to do many additions bellow.
  distance_to_start[start] = 1;
  queue.push_back(start);

  // Standard BFS algorithm
  // See https://en.wikipedia.org/wiki/Breadth-first_search.
  while let Some(current) = queue.pop_front() {
    for &neighbor in graph.get_neighbors(current) {
      if distance_to_start[neighbor] == usize::MAX {
        distance_to_start[neighbor] =
          distance_to_start[current] + 1;
        predecessor_from_start[neighbor] = current;
        queue.push_back(neighbor);
      }
    }
  }

  // The same as above except we set it to 0.
  distance_to_end[end] = 0;
  queue.push_back(end);

  // Standard BFS algorithm
  // See https://en.wikipedia.org/wiki/Breadth-first_search.
  while let Some(current) = queue.pop_front() {
    for &neighbor in graph.get_neighbors(current) {
      if distance_to_end[neighbor] == usize::MAX {
        distance_to_end[neighbor] =
          distance_to_end[current] + 1;
        predecessor_from_end[neighbor] = current;
        queue.push_back(neighbor);
      }
    }
  }

  // The additions mentioned above are here.
  let middle = (0..graph.size).fold(
    (usize::MAX, usize::MAX),
    |middle, vertex| {
      let distance =
        distance_to_start[vertex] + distance_to_end[vertex];

      if closer_to(distance, middle.1, length) {
        (vertex, distance)
      } else {
        middle
      }
    },
  );

  let mut path = Vec::with_capacity(middle.1);

  // Append the path between the middle and the start.
  let mut current = middle.0;

  while current != usize::MAX {
    path.push(current);
    current = predecessor_from_start[current];
  }

  // And then reverse it.
  path.reverse();

  // And then add the path between the middle and the end.
  // We add predecessor_from_end so the middle point isn't
  // added twice.
  current = predecessor_from_end[middle.0];

  while current != usize::MAX {
    path.push(current);
    current = predecessor_from_end[current];
  }

  path
}

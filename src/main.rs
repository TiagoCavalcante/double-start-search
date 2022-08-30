use graphs::Graph;
use std::time::Instant;

mod path;

fn main() {
  let size = 20_000;
  let start = 0;
  let end = 10_000;
  let length = 5;

  let mut graph = Graph::new(size);

  let now = Instant::now();
  graph.fill_undirected(0.01);
  println!("Fill the graph - {:.2?}", now.elapsed());

  let now = Instant::now();
  let path =
    path::double_start_search(&graph, start, end, length);
  println!("Double start search - {:.2?}", now.elapsed());

  // Test if the path is valid.
  assert_eq!(path.len(), length);
  assert_eq!(*path.first().unwrap(), start);
  assert_eq!(*path.last().unwrap(), end);

  // Check if the path is made only by real edges.
  for index in 0..path.len() - 1 {
    assert!(graph.has_edge(path[index], path[index + 1]));
  }

  // Ensure that the path contain no loops.
  let mut unique = path.clone();
  // We need a sorted vector to use dedup.
  unique.sort();
  unique.dedup();
  // If the path had loops then the length of the unique
  // vector would be smaller than the length of the path.
  assert_eq!(path.len(), unique.len());
}

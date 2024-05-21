ok,

so we have a 2D array

```rs
let test_case: Vec<Vec<u8>> = vec![
  vec![1, 0, 0], 
  vec![0, 1, 1], 
  vec![0, 1, 1], 
  vec![1, 1, 0]
];

```

we will count the lakes, and not mutate

lake_count: number;
cells_visited: HashMap<(usize, usize), bool>;


iterate through all of the rows, and the columns for a BREADTH-FIRST

if (val == 0 and not_visited) {
  inc lake_count
  queue: VecDeque<(usize, usize)>;
  queue.push(cur)
  set as visited

  while loop the queue {
    
    bounds check all the neighbours
    for n of neighbours {
      if valid n
      set as visited
      and push to the queue
    }
  }

}

Big O analysis
- time complexity is linear time, since we need to walk through all the nodes
- If every node was a lake we would technically be going over each node twice. So at most 2N time, which is still N time.
- Space complexity is linear, and could be constant by mutating.

# double-start-search

Approximated length search in O(V + E)

## Where did this come from?

This is a more serious version of [approximate-length-search](https://github.com/TiagoCavalcante/approximate-length-search) and [approximate-cost-search](https://github.com/TiagoCavalcante/approximate-cost-search), still, it only gives a good approximation when the desired length is close to the length of the smallest path between the start and the end.

This is slower than BFS but its time complexity is the same. This is because of how big-O works:  
O((V + E) + (V + E) + V) = O(3 V + 2 E) = O(V + E)

## How fast is it?

The results for a graph with 20 thousand vertices and density 0.01:
```
Fill the graph - 1.35s
Double start search - 20.13ms
```

## Better approach?

You can find the state-of-art for unweighted graphs [here](https://github.com/TiagoCavalcante/fixed-length-search), and what I believe it's the state-of-art for weighted graphs [here](https://github.com/TiagoCavalcante/fixed-cost-search).

use crate::ds::heap::min_heap::MinHeap;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct NodeInfo {
    index: usize,
    distance: usize,
}

pub fn dijkstra(graph: &Vec<Vec<i32>>, source: usize) -> Vec<usize> {
    let node_num = graph.len();
    let mut distance = vec![usize::MAX; node_num];
    distance[source] = 0;

    let mut heap = MinHeap::new();
    heap.insert(NodeInfo {
        index: source,
        distance: 0,
    });

    while heap.size() != 0 {
        let node = heap.extract_min().unwrap();
        let adjacency_list = find_adjacency(&graph, node.index);
        adjacency_list.iter().for_each(|&adjacency| {
            let d_source_node = distance[node.index];
            let d_node_adjacency = get_distance(&graph, node.index, adjacency);
            if d_source_node + d_node_adjacency < distance[adjacency] {
                distance[adjacency] = d_source_node + d_node_adjacency;
                heap.insert(NodeInfo {
                    index: adjacency,
                    distance: distance[adjacency],
                });
            }
        });
    }
    distance
}

fn get_distance(graph: &Vec<Vec<i32>>, src: usize, des: usize) -> usize {
    let distance = graph[src][des];
    if distance >= 0 {
        distance as usize
    } else {
        usize::MAX
    }
}

fn find_adjacency(graph: &Vec<Vec<i32>>, src: usize) -> Vec<usize> {
    let mut adjencency_list = vec![];
    for (node, &distance) in graph[src].iter().enumerate() {
        if distance >= 0 {
            adjencency_list.push(node);
        }
    }
    adjencency_list
}

#[cfg(test)]
mod tests {

    use crate::algo::graph;

    use super::*;

    #[test]
    fn simple_test() {
        let graph = vec![
            vec![0, 2, 4, -1],
            vec![2, 0, -1, 1],
            vec![4, -1, 0, 3],
            vec![-1, 1, 3, 0],
        ];
        assert_eq!(dijkstra(&graph, 0), vec![0, 2, 4, 3]);
    }

    #[test]
    fn multi_shortest() {
        let graph = vec![
            vec![0, 1, 4, 1],
            vec![1, 0, -1, 2],
            vec![4, -1, 0, 3],
            vec![1, 2, 3, 0],
        ];
        assert_eq!(dijkstra(&graph, 0), vec![0, 1, 4, 1]);
    }

    #[test]
    fn complex_test() {
        let graph = vec![
            vec![0, 1, 4, -1, -1, -1],
            vec![1, 0, -1, 2, -1, 5],
            vec![4, -1, 0, 3, 6, -1],
            vec![-1, 2, 3, 0, -1, 1],
            vec![-1, -1, 6, -1, 0, -1],
            vec![-1, 5, -1, 1, -1, 0],
        ];
        assert_eq!(dijkstra(&graph, 0), vec![0, 1, 4, 3, 10, 4]);
    }

    #[test]
    fn complex_test_2() {
        let graph = vec![
            vec![0, 0, 0, 43, 91],
            vec![1, 0, 59, 23, 4],
            vec![36, 98, 0, 22, 78],
            vec![16, 51, 59, 0, 75],
            vec![15, 22, 85, 31, 0],
        ];
        assert_eq!(dijkstra(&graph, 4), vec![15, 15, 15, 31, 0]);
    }
}

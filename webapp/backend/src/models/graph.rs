use sqlx::FromRow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}
#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}
#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}
impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());
        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }
    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = HashMap::new(); // 各ノードまでの最短距離を保持
        let mut heap = BinaryHeap::new(); // 優先度付きキュー
        // 開始ノードの初期化
        distances.insert(from_node_id, 0);
        heap.push(State {
            cost: 0,
            node_id: from_node_id,
        });
        // ダイクストラ法で探索
        while let Some(State { cost, node_id }) = heap.pop() {
            // 既により短い距離が見つかっていれば無視
            if let Some(&current_distance) = distances.get(&node_id) {
                if cost > current_distance {
                    continue;
                }
            }
            // 目的ノードに到達したら、その時点の距離を返す
            if node_id == to_node_id {
                return cost;
            }
            // エッジをすべて探索
            if let Some(edges) = self.edges.get(&node_id) {
                for edge in edges {
                    let next_node_id = edge.node_b_id;
                    let next_cost = cost + edge.weight;
                    // 新しい距離が既存の距離よりも短ければ更新
                    if next_cost < *distances.get(&next_node_id).unwrap_or(&i32::MAX) {
                        distances.insert(next_node_id, next_cost);
                        heap.push(State {
                            cost: next_cost,
                            node_id: next_node_id,
                        });
                    }
                }
            }
        }
        // 目的地に到達できなかった場合、最大値を返す
        i32::MAX
    }
}
// ヒープで使用するためのアイテム（距離とノードIDのペア）
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    node_id: i32,
}
// コストの低い順に処理するための Ord の実装
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // 逆順にすることで最小のコストを先に処理
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
use bevy::utils::HashMap;
use priority_queue::PriorityQueue;
use std::hash::Hash;

pub trait Pathfind<Node: Vertex + Hash + Eq> {
    fn find_path(&self, start: Node, end: Node) -> Option<Vec<Node>> {
        let verticies = self.get_all_verticies();

        let mut prev: HashMap<usize, usize> = HashMap::new();
        // Because we need a min-queue and this priority queue is a max-queue we use the negative distance
        let mut queue = PriorityQueue::with_capacity(verticies.len());

        let end_id = end.get_id();

        queue.push(start, 0);

        'main_loop: loop {
            let Some((u, dist)) = queue.pop() else {
                return None;
            };

            let curr_id = u.get_id();
            for neighbour in self.get_neighbours(&u) {
                if prev.contains_key(&neighbour.get_id()) {
                    continue;
                }

                prev.insert(neighbour.get_id(), curr_id);

                if neighbour.get_id() == end_id {
                    break 'main_loop;
                }

                queue.push(neighbour, dist - 1);
            }
        }

        let mut verticies = verticies
            .into_iter()
            .map(|v| (v.get_id(), v))
            .collect::<HashMap<usize, Node>>();
        let mut path = vec![end];
        let mut id = &end_id;
        while let Some(prev_id) = prev.get(id) {
            if prev_id == &0 {
                break;
            }

            path.push(
                verticies
                    .remove(prev_id)
                    .expect("Verticies did not contain ID"),
            );
            id = prev_id;
        }

        path.reverse();
        Some(path)
    }

    // TODO: Do some fancy lifetime stuff with the nodes here
    fn get_all_verticies(&self) -> Vec<Node>;

    fn get_neighbours(&self, vertex: &Node) -> Vec<Node>;
}

pub trait Vertex {
    fn get_id(&self) -> usize;
}

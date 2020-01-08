use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Classification {
    Acyclic,
    Cyclic,
}

pub struct Graph<T>
where
    T: Eq + Hash + Clone,
{
    nodes: HashSet<T>,
    edges: HashMap<T, Vec<T>>,
}

#[allow(dead_code)]
impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    pub fn from_edge_map(mut edges: HashMap<T, Vec<T>>) -> Self {
        let nodes1 = edges.iter().map(|(node, _)| node.clone());
        let nodes2 = edges.iter().flat_map(|(_, edges)| edges.iter().cloned());
        let nodes: HashSet<T> = nodes1.chain(nodes2).collect();
        for node in nodes.iter() {
            if !edges.contains_key(node) {
                edges.insert(node.clone(), Vec::new());
            }
        }
        Graph { nodes, edges }
    }

    pub fn nodes(&self) -> &HashSet<T> {
        &self.nodes
    }

    pub fn edges(&self) -> &HashMap<T, Vec<T>> {
        &self.edges
    }

    pub fn linearization(&self) -> Option<Vec<T>> {
        let mut list = Vec::<T>::new();
        let mut pre_visit = |_: &T| ();
        let mut post_visit = |t: &T| list.push(t.clone());
        if self.depth_first_search(&mut pre_visit, &mut post_visit) == Classification::Cyclic {
            return None;
        }
        list.reverse();
        Some(list)
    }

    pub fn edges_from(&self, t: &T) -> Option<&Vec<T>> {
        self.edges.get(t)
    }

    pub fn pre_visit_nums(&self) -> HashMap<T, usize> {
        let mut nums = HashMap::new();
        let mut cur_num = 0;
        let mut pre_visit = |t: &T| {
            nums.insert(t.clone(), cur_num);
            cur_num += 1;
        };
        let mut post_visit = |_: &T| ();
        self.depth_first_search(&mut pre_visit, &mut post_visit);
        nums
    }

    pub fn post_visit_nums(&self) -> HashMap<T, usize> {
        let mut nums = HashMap::new();
        let mut cur_num = 0;
        let mut pre_visit = |_: &T| ();
        let mut post_visit = |t: &T| {
            nums.insert(t.clone(), cur_num);
            cur_num += 1;
        };
        self.depth_first_search(&mut pre_visit, &mut post_visit);
        nums
    }

    pub fn depth_first_search<PreF, PostF>(
        &self,
        pre_visit: &mut PreF,
        post_visit: &mut PostF,
    ) -> Classification
    where
        PreF: FnMut(&T),
        PostF: FnMut(&T),
    {
        let mut classification = Classification::Acyclic;
        let mut visited = HashSet::<&T>::new();
        let mut active = HashSet::<&T>::new();
        for node in self.nodes() {
            if !visited.contains(node) {
                if self.explore(node, &mut visited, &mut active, pre_visit, post_visit)
                    == Classification::Cyclic
                {
                    classification = Classification::Cyclic;
                }
            }
        }
        classification
    }

    fn explore<'a, PreF, PostF>(
        &'a self,
        item: &'a T,
        visited: &mut HashSet<&'a T>,
        active: &mut HashSet<&'a T>,
        pre_visit: &mut PreF,
        post_visit: &mut PostF,
    ) -> Classification
    where
        PreF: FnMut(&T),
        PostF: FnMut(&T),
    {
        let mut classification = Classification::Acyclic;
        visited.insert(item);
        active.insert(item);
        pre_visit(item);
        for next in self.edges_from(item).expect("Invalid node in explore") {
            if active.contains(next) {
                classification = Classification::Cyclic;
            }
            if !visited.contains(next) {
                self.explore(next, visited, active, pre_visit, post_visit);
            }
        }
        post_visit(item);
        active.remove(item);
        classification
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_edges() {
        let edges = vec![('a', vec!['b', 'c']), ('b', vec!['c'])]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let graph = Graph::from_edge_map(edges);
        assert_eq!(graph.nodes(), &vec!['a', 'b', 'c'].into_iter().collect());
        let all_edges = vec![('a', vec!['b', 'c']), ('b', vec!['c']), ('c', vec![])]
            .into_iter()
            .collect::<HashMap<_, _>>();
        assert_eq!(graph.edges(), &all_edges);
    }

    #[test]
    fn test_depth_first_search() {
        type T = char;
        let edges = vec![('a', vec!['b', 'c']), ('b', vec!['c'])]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let graph = Graph::from_edge_map(edges.clone());
        let mut post_visits = Vec::new();
        let mut pre_visit = |_: &T| ();
        let mut post_visit = |&c: &T| post_visits.push(c);
        graph.depth_first_search(&mut pre_visit, &mut post_visit);
        assert_eq!(post_visits, vec!['c', 'b', 'a']);
    }

    #[test]
    fn test_post_visit_nums() {
        let edges = vec![('a', vec!['b', 'c']), ('b', vec!['c'])]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let graph = Graph::from_edge_map(edges.clone());
        let post_visit_nums = graph.post_visit_nums();
        assert_eq!(
            post_visit_nums,
            vec![('a', 2), ('b', 1), ('c', 0)].into_iter().collect()
        );
    }

    #[test]
    fn test_linearization() {
        let edges = vec![('a', vec!['b', 'c']), ('b', vec!['c'])]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let graph = Graph::from_edge_map(edges.clone());
        let list = graph.linearization();
        assert_eq!(list, Some(vec!['a', 'b', 'c']));
    }
}

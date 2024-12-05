use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

pub type ReachabilityMap = HashMap<u32, RefCell<HashSet<u32>>>;

pub struct Rules {
    pub rules: Vec<(u32, u32)>,
}

impl Rules {
    // It turns out no update contains pages that don't have any ordering rules
    // If they did, we could ignore them out by filtering against this
    pub fn vertices(&self) -> HashSet<u32> {
        self.rules.iter().flat_map(|&(a, b)| [a, b]).collect()
    }

    // This interprets our poset as a DAG, where having a rule a|b is having an edge a->b
    // But confusingly, it is not globally a DAG, only the rules used locally in an update are!
    // So we don't try to consider paths from a to b here, only direct edges
    pub fn direct_reachability(&self) -> ReachabilityMap {
        let reachability: ReachabilityMap = self
            .vertices()
            .iter()
            .map(|&v| (v, Default::default()))
            .collect();
        for &(a, b) in &self.rules {
            reachability[&a].borrow_mut().insert(b);
        }
        reachability
    }

    // This interprets our poset as a DAG, where if there is a path from a to b, then a < b
    // If a|b and b|c, we consider that a|c. However, this is not correct =)
    // The input rules may globally contain cycles (just not in a particular update!)
    #[allow(unused)]
    pub fn reachability(&self) -> ReachabilityMap {
        let vertices = self.vertices();
        let mut reachability: ReachabilityMap =
            vertices.iter().map(|&v| (v, Default::default())).collect();
        for &(a, b) in &self.rules {
            let mut targets = reachability[&b].borrow().clone();
            targets.insert(b);

            // a can reach b, and what b can reach
            reachability[&a].borrow_mut().extend(targets.iter());

            // So can predecessors of a
            for reach in &mut reachability.values_mut() {
                let mut reach = reach.borrow_mut();
                if reach.contains(&a) {
                    reach.extend(targets.iter());
                }
            }
        }
        reachability
    }
}

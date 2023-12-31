use crate::cfg::{graph_from_function, Dir, CFG};
use crate::utils::CFGNode;
use bril_rs::Program;
use petgraph::graph::NodeIndex;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn union<T>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    a.union(b).cloned().collect()
}

fn intersection<T>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    a.intersection(b).cloned().collect()
}

fn live_var_transfer<T: CFGNode + Clone + Debug>(
    l: &HashSet<String>,
    n: &NodeIndex,
    cfg: &CFG<T>,
) -> HashSet<String> {
    let n = cfg.graph.node_weight(*n).unwrap();
    n.uses().union(&(l - &n.defs())).cloned().collect()
}

pub fn live_variable_analysis<T: CFGNode + Clone + std::fmt::Debug + std::fmt::Display>(
    cfg: &CFG<T>,
) -> (Vec<HashSet<String>>, Vec<HashSet<String>>) {
    cfg.work_list(
        union,
        live_var_transfer,
        HashSet::<String>::new(),
        HashSet::<String>::new(),
        Dir::Backward,
    )
}

pub fn live_variable_debug(prog: &Program) {
    for func in &prog.functions {
        let cfg = graph_from_function(func);
        let (entry, exit) = live_variable_analysis(&cfg);
        println!("@{} Liveness Analysis", func.name);
        for i in 0..cfg.graph.node_count() {
            println!(
                "{:?}:",
                cfg.graph
                    .node_weight(NodeIndex::from(i as u32))
                    .unwrap()
                    .label
            );
            println!("Entry: {:?}", entry[i]);
            println!("Exit: {:?}", exit[i]);
            println!("");
        }
    }
}

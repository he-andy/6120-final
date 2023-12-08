use std::collections::HashSet;

use bril_rs::Code;
use petgraph::graph::NodeIndex;

use crate::{
    cfg::CFG,
    dominator,
    utils::{BasicBlock, CFGNode},
};

impl<T: CFGNode + Clone + std::fmt::Debug + std::fmt::Display> CFG<T> {
    pub fn small_natural_loops(&self) -> Vec<[NodeIndex; 2]> {
        let doms = dominator::dominator_analyis(&self).0; //get dominators
        let mut loops = Vec::new();
        for node in self.graph.node_indices() {
            for idom in doms.immediately_dominated_by(node) {
                if idom != node {
                    if self.graph.contains_edge(idom, node) {
                        loops.push([idom, node]);
                    }
                }
            }
        }

        loops
    }
}

pub fn remove_data_dependencies(bb: &BasicBlock, fn_name: &String, bb_idx: usize) -> Vec<Code> {
    let mut code = Vec::new();
    let mut uses = HashSet::new();
    let mut i = 0;
    for instr in bb.instructions.iter().rev() {
        uses.extend(instr.uses());
        if instr.defs().iter().any(|def| uses.contains(def)) {
            code.push(Code::Label {
                label: format!("_par_block_fn{}_bb{}_{}_", fn_name, bb_idx, i),
                pos: None,
            });
            uses.clear();
            uses.extend(instr.uses());
        }
        code.push(instr.clone());
        i += 1
    }
    code.push(Code::Label {
        label: format!("_par_block_fn{}_bb{}_{}_", fn_name, bb_idx, i),
        pos: None,
    });
    code.reverse();
    code
}

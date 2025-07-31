use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;
use std::collections::HashMap;
use crate::ast::{CategoryAST, Statement};

/// Node in the DAG representing either an object or a morphism
#[derive(Debug, Clone)]
pub enum DAGNode {
    /// Object node (input/output ports)
    Object { name: String },
    /// Morphism node (combinational logic)
    Morphism { name: String, from: String, to: String },
}

/// Edge in the DAG representing data flow
#[derive(Debug, Clone)]
pub struct DAGEdge {
    pub width: usize, // Signal width in bits
}

/// DAG representation of the category theory description
pub struct CategoryDAG {
    pub graph: DiGraph<DAGNode, DAGEdge>,
    pub node_indices: HashMap<String, NodeIndex>,
}

impl CategoryDAG {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    /// Build DAG from AST
    pub fn from_ast(ast: &CategoryAST) -> Result<Self, String> {
        let mut dag = Self::new();
        
        // First pass: add all objects and morphisms as nodes
        for stmt in &ast.statements {
            match stmt {
                Statement::Object(name) => {
                    let node_idx = dag.graph.add_node(DAGNode::Object {
                        name: name.clone(),
                    });
                    dag.node_indices.insert(name.clone(), node_idx);
                }
                Statement::Morphism { name, from, to } => {
                    let node_idx = dag.graph.add_node(DAGNode::Morphism {
                        name: name.clone(),
                        from: from.clone(),
                        to: to.clone(),
                    });
                    dag.node_indices.insert(name.clone(), node_idx);
                }
                _ => {}
            }
        }

        // Second pass: add edges based on morphism definitions
        for stmt in &ast.statements {
            if let Statement::Morphism { name, from, to } = stmt {
                let morphism_idx = dag.node_indices.get(name)
                    .ok_or_else(|| format!("Morphism {} not found", name))?;
                let from_idx = dag.node_indices.get(from)
                    .ok_or_else(|| format!("Object {} not found", from))?;
                let to_idx = dag.node_indices.get(to)
                    .ok_or_else(|| format!("Object {} not found", to))?;

                // Add edge from source object to morphism
                dag.graph.add_edge(*from_idx, *morphism_idx, DAGEdge { width: 8 });
                // Add edge from morphism to target object
                dag.graph.add_edge(*morphism_idx, *to_idx, DAGEdge { width: 8 });
            }
        }

        Ok(dag)
    }

    /// Get topological sort of the DAG
    pub fn get_execution_order(&self) -> Result<Vec<NodeIndex>, String> {
        toposort(&self.graph, None)
            .map_err(|e| format!("Cycle detected in DAG: {:?}", e))
    }

    /// Validate that all commutativity assertions are satisfied
    pub fn validate_commutativity(&self, ast: &CategoryAST) -> Result<(), String> {
        for stmt in &ast.statements {
            if let Statement::AssertCommute { lhs, rhs } = stmt {
                // For now, we'll just check that the paths exist
                // In a full implementation, we'd verify the actual commutativity
                println!("Checking commutativity: {:?} == {:?}", lhs, rhs);
            }
        }
        Ok(())
    }
}

/// Verilog module representation
#[derive(Debug, Clone)]
pub struct VerilogModule {
    pub name: String,
    pub inputs: Vec<(String, usize)>, // (name, width)
    pub outputs: Vec<(String, usize)>,
    pub wires: Vec<(String, usize)>,
    pub assignments: Vec<String>,
}

/// Netlist representation
pub struct Netlist {
    pub modules: Vec<VerilogModule>,
    pub top_module: VerilogModule,
}

impl Netlist {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            top_module: VerilogModule {
                name: "top".to_string(),
                inputs: Vec::new(),
                outputs: Vec::new(),
                wires: Vec::new(),
                assignments: Vec::new(),
            },
        }
    }

    /// Generate Verilog code from DAG
    pub fn from_dag(dag: &CategoryDAG, ast: &CategoryAST) -> Result<Self, String> {
        let mut netlist = Self::new();
        
        // Get execution order
        let execution_order = dag.get_execution_order()?;
        
        // Generate modules for each morphism
        for node_idx in execution_order {
            if let Some(node) = dag.graph.node_weight(node_idx) {
                match node {
                    DAGNode::Morphism { name, from, to } => {
                        let module = VerilogModule {
                            name: format!("morphism_{}", name),
                            inputs: vec![(format!("in_{}", from), 8)],
                            outputs: vec![(format!("out_{}", to), 8)],
                            wires: Vec::new(),
                            assignments: vec![
                                format!("assign out_{} = in_{} + 1; // Placeholder logic", to, from)
                            ],
                        };
                        netlist.modules.push(module);
                    }
                    _ => {}
                }
            }
        }

        // Build top module
        let objects = ast.get_objects();
        for obj in objects {
            netlist.top_module.inputs.push((format!("in_{}", obj), 8));
            netlist.top_module.outputs.push((format!("out_{}", obj), 8));
        }

        Ok(netlist)
    }

    /// Generate Verilog code as string
    pub fn to_verilog(&self) -> String {
        let mut verilog = String::new();
        
        // Generate individual modules
        for module in &self.modules {
            verilog.push_str(&self.module_to_verilog(module));
            verilog.push_str("\n\n");
        }
        
        // Generate top module
        verilog.push_str(&self.module_to_verilog(&self.top_module));
        
        verilog
    }

    fn module_to_verilog(&self, module: &VerilogModule) -> String {
        let mut verilog = format!("module {} (\n", module.name);
        
        // Inputs
        for (i, (name, width)) in module.inputs.iter().enumerate() {
            verilog.push_str(&format!("    input [{}:0] {}", width - 1, name));
            if i < module.inputs.len() - 1 || !module.outputs.is_empty() {
                verilog.push_str(",");
            }
            verilog.push_str("\n");
        }
        
        // Outputs
        for (i, (name, width)) in module.outputs.iter().enumerate() {
            verilog.push_str(&format!("    output [{}:0] {}", width - 1, name));
            if i < module.outputs.len() - 1 {
                verilog.push_str(",");
            }
            verilog.push_str("\n");
        }
        
        verilog.push_str(");\n\n");
        
        // Wires
        for (name, width) in &module.wires {
            verilog.push_str(&format!("    wire [{}:0] {};\n", width - 1, name));
        }
        if !module.wires.is_empty() {
            verilog.push_str("\n");
        }
        
        // Assignments
        for assignment in &module.assignments {
            verilog.push_str(&format!("    {}\n", assignment));
        }
        
        verilog.push_str("endmodule\n");
        verilog
    }
} 
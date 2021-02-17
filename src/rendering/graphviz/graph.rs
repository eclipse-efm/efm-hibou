/*******************************************************************************
* Copyright (c) 2021 CentraleSupelec, CEA-LIST
*
* This program and the accompanying materials
* are made available under the terms of the Eclipse Public License 2.0
* which accompanies this distribution, and is available at
* https://www.eclipse.org/legal/epl-2.0/
*
* SPDX-License-Identifier: EPL-2.0
*
* Contributors:
* Erwan Mahé (CentraleSupelec) - initial API and implementation
*******************************************************************************/


use crate::rendering::graphviz::node_style::GraphvizNodeStyle;
use crate::rendering::graphviz::edge_style::GraphvizEdgeStyle;
use crate::rendering::graphviz::common::DotTranslatable;

pub struct GraphVizNode {
    pub id : String,
    pub style : GraphvizNodeStyle
}


impl DotTranslatable for GraphVizNode {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        res.push_str(&(self.id));
        res.push_str(&(self.style.to_dot_string()));
        res.push_str(";");
        return res;
    }
}


pub struct GraphVizEdge {
    pub origin_id : String,
    pub target_id : String,
    pub style : GraphvizEdgeStyle
}

impl DotTranslatable for GraphVizEdge {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        res.push_str(&(self.origin_id));
        res.push_str("->");
        res.push_str(&(self.target_id));
        res.push_str(& self.style.to_dot_string() );
        res.push_str(";");
        return res;
    }
}


pub struct GraphVizDiGraph {
    pub nodes : Vec<GraphVizNode>,
    pub edges : Vec<GraphVizEdge>
}

impl DotTranslatable for GraphVizDiGraph {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        res.push_str("digraph G {");
        for node in &self.nodes {
            res.push_str("\n\t");
            res.push_str(& node.to_dot_string() );
        }
        for edge in &self.edges {
            res.push_str("\n\t");
            res.push_str(& edge.to_dot_string() );
        }
        res.push_str("\n}");
        return res;
    }
}
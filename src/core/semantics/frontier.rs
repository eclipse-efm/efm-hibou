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

use std::collections::HashSet;

use crate::core::syntax::position::*;
use crate::core::syntax::interaction::*;



pub fn make_frontier(interaction : &Interaction) -> Vec<Position> {
    match interaction {
        &Interaction::Empty => {
            return Vec::new();
        },
        &Interaction::Action(_) => {
            return vec![Position::Epsilon];
        },
        &Interaction::Strict(ref i1, ref i2) => {
            let mut front = push_frontier(&PositionKind::Left, make_frontier(i1));
            if i1.express_empty() {
                front.append( &mut push_frontier(&PositionKind::Right, make_frontier(i2)) )
            }
            return front;
        },
        &Interaction::Seq(ref i1, ref i2) => {
            let mut front = push_frontier(&PositionKind::Left, make_frontier(i1));
            for pos2 in push_frontier(&PositionKind::Right, make_frontier(i2)) {
                let act = interaction.get_sub_interaction(&pos2 ).as_leaf();
                if i1.avoids(act.lf_act.lf_id) {
                    front.push(pos2);
                }
            }
            return front;
        },
        &Interaction::Alt(ref i1, ref i2) => {
            let mut front = push_frontier(&PositionKind::Left, make_frontier(i1));
            front.append( &mut push_frontier(&PositionKind::Right, make_frontier(i2)) );
            return front;
        },
        &Interaction::Par(ref i1, ref i2) => {
            let mut front = push_frontier(&PositionKind::Left, make_frontier(i1));
            front.append( &mut push_frontier(&PositionKind::Right, make_frontier(i2)) );
            return front;
        },
        &Interaction::Loop(_, ref i1) => {
            return push_frontier(&PositionKind::Left, make_frontier(i1));
        },
        &Interaction::Scope(_, ref i1) => {
            return push_frontier(&PositionKind::Left, make_frontier(i1));
        }
    }
}



enum PositionKind {
    Left,
    Right
}

fn push_frontier(pkind : &PositionKind, frontier : Vec<Position>) -> Vec<Position> {
    let mut new_frontier : Vec<Position> = Vec::new();
    // ***
    for my_pos in frontier {
        let new_pos : Position;
        match pkind {
            PositionKind::Left => {
                new_pos = Position::Left( Box::new(my_pos ) );
            },
            PositionKind::Right => {
                new_pos = Position::Right( Box::new(my_pos ) );
            }
        }
        new_frontier.push( new_pos );
    }
    // ***
    return new_frontier;
}



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

use crate::core::syntax::interaction::Interaction;


pub fn get_recursive_alt_frags(interaction : &Interaction) -> Vec<&Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            frags.extend( get_recursive_alt_frags(i1));
            frags.extend( get_recursive_alt_frags(i2));
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}

pub fn get_recursive_par_frags(interaction : &Interaction) -> Vec<&Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::Par(ref i1, ref i2) => {
            frags.extend( get_recursive_par_frags(i1));
            frags.extend( get_recursive_par_frags(i2));
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}

pub fn get_recursive_strict_frags(interaction : &Interaction) -> Vec<&Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            frags.extend( get_recursive_strict_frags(i1));
            frags.extend( get_recursive_strict_frags(i2));
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}


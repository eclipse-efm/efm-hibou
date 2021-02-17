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

use pest::iterators::Pair;

use crate::from_text::parser::*;
use crate::core::syntax::data::td_type::TD_DataType;

pub fn parse_type(type_pair : Pair<Rule>) -> (TD_DataType,bool) {
    match type_pair.as_rule() {
        Rule::DATA_TYPE_Clock => {
            return (TD_DataType::Float,true);
        },
        Rule::DATA_TYPE_Bool => {
            return (TD_DataType::Bool,false);
        },
        Rule::DATA_TYPE_Integer => {
            return (TD_DataType::Integer,false);
        },
        Rule::DATA_TYPE_Float => {
            return (TD_DataType::Float,false);
        },
        Rule::DATA_TYPE_String => {
            return (TD_DataType::String,false);
        },
        _ => {
            panic!("what rule then ? : {:?}", type_pair.as_rule() );
        }
    }
}
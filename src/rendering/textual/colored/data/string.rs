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
* Erwan Mah� (CentraleSupelec) - initial API and implementation
*******************************************************************************/

use image::Rgb;

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::data::builtin::string::TD_String;

use crate::rendering::hibou_color_palette::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::textual::convention::*;

impl ColoredTextable for TD_String {
    fn to_colored_text(&self, gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
        match self {
            TD_String::Value( str_val ) => {
                let mut texts : Vec<TextToPrint> = Vec::new();
                texts.push( TextToPrint{text:"\"".to_string(),color:Rgb(HC_Grammar_Symbol)});
                texts.push( TextToPrint{text:str_val.to_string(),color:Rgb(HC_Concrete_Value)} );
                texts.push( TextToPrint{text:"\"".to_string(),color:Rgb(HC_Grammar_Symbol)});
                return texts;
            },
            TD_String::Reference( var_ref ) => {
                return var_ref.to_colored_text(gen_ctx,exe_ctx);
            }
        }
    }
}
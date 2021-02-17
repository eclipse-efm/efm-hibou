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

use std::collections::HashMap;

// ***
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::core::syntax::position::*;
use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::action::*;
use crate::core::context::general::GeneralContext;

use crate::core::context::execution::ExecutionContext;
use crate::core::trace::*;


use crate::process::hibou_process::FilterEliminationKind;

use crate::process::verdicts::*;



pub trait ProcessLogger {

    fn log_init(&mut self,
                interaction : &Interaction,
                gen_ctx : &GeneralContext,
                exe_ctx : &ExecutionContext,
                remaining_multi_trace : &Option<AnalysableMultiTrace>);

    fn log_term(&mut self,
                options_as_str : &Vec<String>);

    fn log_execution(&mut self,
                     gen_ctx : &GeneralContext,
                     parent_state_id : u32,
                     new_state_id : u32,
                     action_position : &Position,
                     trace_action : Option<&TraceAction>,
                     model_action : &ObservableAction,
                     new_interaction : &Interaction,
                     new_exe_ctx : &ExecutionContext,
                     remaining_multi_trace : &Option<AnalysableMultiTrace>);

    fn log_verdict(&mut self,
                   parent_state_id : u32,
                   verdict : &CoverageVerdict);

    fn log_filtered(&mut self,
                    gen_ctx : &GeneralContext,
                    exe_ctx:&ExecutionContext,
                    parent_state_id : u32,
                    new_state_id : u32,
                    action_position : &Position,
                    action : &ObservableAction,
                    elim_kind : &FilterEliminationKind);

    fn log_unsat(&mut self,
                 gen_ctx : &GeneralContext,
                 exe_ctx:&ExecutionContext,
                 parent_state_id : u32,
                 new_state_id : u32,
                 action_position : &Position,
                 trace_action : Option<&TraceAction>,
                 model_action : &ObservableAction);
}


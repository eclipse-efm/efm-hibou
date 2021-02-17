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
use std::fs::write;
use std::path::Path;
use clap::App;
use clap::ArgMatches;


use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::trace::*;




use crate::rendering::custom_draw::seqdiag::interaction::draw_interaction;


use crate::rendering::process::graphic_logger::GraphicProcessLogger;

use crate::process::log::*;
use crate::process::exploration::explore;
use crate::process::analysis::analyze;
use crate::from_text::hsf_file::{ProcessKind,parse_hsf_file};
use crate::from_text::htf_file::parse_htf_file;
use crate::process::hibou_process::HibouSearchStrategy;



pub async fn test_explore(name : &str) {
    let hsf_file_path = name.to_string();
    match parse_hsf_file(&hsf_file_path, &ProcessKind::Explore) {
        Err(e) => {
            println!("{}", "error parsing .hsf file".to_string() );
            println!("{}", e.to_string() );
        },
        Ok( (gen_ctx,exe_ctx,my_int,mut hoptions) ) => {
            explore(my_int, gen_ctx, exe_ctx,hoptions.temporality,hoptions.pre_filters, hoptions.strategy,hoptions.frontier_priorities,hoptions.loggers).await;
        }
    }
}

pub async fn test_analyze(hsfname : &str, htfname : &str) {
    let hsf_file_path = hsfname.to_string();
    match parse_hsf_file(&hsf_file_path,&ProcessKind::Analyze) {
        Err(e) => {
            println!("{}", "error parsing .hsf file".to_string() );
            println!("{}", e.to_string() );
        },
        Ok( (gen_ctx,exe_ctx,my_int,mut hoptions) ) => {
            let htf_file_path = htfname.to_string();
            match parse_htf_file(&htf_file_path,&gen_ctx,&hoptions.temporality) {
                Err(e) => {
                    println!("{}", "error parsing .htf file".to_string() );
                    println!("{}", e.to_string() );
                },
                Ok( multi_trace ) => {
                    let verdict = analyze(my_int,
                            multi_trace,
                            gen_ctx,
                            exe_ctx,
                            hoptions.temporality,
                            hoptions.pre_filters,
                            hoptions.strategy,
                            hoptions.frontier_priorities,
                            hoptions.loggers,
                            hoptions.goal.unwrap()).await;
                    println!("{}", format!("got verdict {:}", verdict.to_string()));
                }
            }
        }
    }
}


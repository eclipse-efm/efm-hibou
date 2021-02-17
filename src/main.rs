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

extern crate strum;

#[macro_use]
extern crate strum_macros;

extern crate rusttype;

extern crate image;

extern crate imageproc;

extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate clap;

extern crate tonic;

extern crate prost;

extern crate bytes;

// **********

pub mod tools;
pub mod core;
pub mod from_text;
pub mod rendering;
pub mod process;
pub mod ui;
pub mod diversity;
pub mod grpc_connect;
pub mod xlia;
// **********

use crate::ui::hibou_cli::hibou_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    hibou_cli().await;
    return Ok(());
}



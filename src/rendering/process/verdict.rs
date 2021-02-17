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


use crate::process::verdicts::CoverageVerdict;

use crate::rendering::graphviz::common::GraphvizColor;


impl CoverageVerdict {
    pub fn get_verdict_color(&self) -> GraphvizColor {
        match self {
            CoverageVerdict::Cov => {
                return GraphvizColor::blue3;
            },
            CoverageVerdict::TooShort => {
                return GraphvizColor::cyan3;
            },
            CoverageVerdict::LackObs => {
                return GraphvizColor::orangered3;
            },
            CoverageVerdict::Out => {
                return GraphvizColor::red3;
            }
        }
    }
}






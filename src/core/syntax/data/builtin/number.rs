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

#[derive(Clone, PartialEq, Debug)]
pub enum ARITH_ADD_SIGN {
    Plus,
    Minus
}

#[derive(Clone, PartialEq, Debug)]
pub enum ARITH_FACTOR_SIGN {
    Mult,
    Div
}


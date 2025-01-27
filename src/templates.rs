// Library for automatic generation of telegram bot api implementation for rust.
// Copyright (C) 2025  Andrew Kozmin <syn.kolbasyn.06@gmail.com>
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


use std::collections::HashMap;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub(crate) struct Api {
    pub(crate) version: String,
    pub(crate) release_date: String,
    pub(crate) changelog: String,
    pub(crate) types: HashMap<String, Type>,
    pub(crate) methods: HashMap<String, Method>,
}


#[derive(Debug, Deserialize)]
pub(crate) struct Type {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) description: Vec<String>,
    pub(crate) fields: Option<Vec<Field>>,
    pub(crate) subtypes: Option<Vec<String>>,
    pub(crate) subtype_of: Option<Vec<String>>,
}


#[derive(Debug, Deserialize)]
pub(crate) struct Method {
    pub(crate) name: String,
    pub(crate) href: String,
    pub(crate) description: Vec<String>,
    pub(crate) returns: Vec<String>,
    pub(crate) fields: Option<Vec<Field>>,
}


#[derive(Debug, Deserialize)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) types: Vec<String>,
    pub(crate) required: bool,
    pub(crate) description: String,
}

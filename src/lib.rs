// Library for automatic generation of telegram bot api implementation for rust.
// Copyright (C) 2025  Andrew Kozmin
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


mod templates;


use anyhow::Result;
use reqwest::{get, Response};

use crate::templates::Api;


pub async fn generate() -> Result<()> {
    let response: Response = get("https://raw.githubusercontent.com/PaulSonOfLars/telegram-bot-api-spec/refs/heads/main/api.min.json").await?;
    let api: Api = response.json().await?;
    Ok(())
}

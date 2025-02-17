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


use anyhow::Result;

use tg_bot_api::generate;


#[tokio::test]
async fn main() -> Result<()>{
    generate().await?;
    Ok(())
}

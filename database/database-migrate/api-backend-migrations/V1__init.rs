/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # API Backend Database Migration V1: Schema Initialization

use barrel::Migration;
use barrel::backend::Pg;
use barrel::types;

/// Perform the database migration.
pub fn migration() -> String {
    let mut migration = Migration::new();
    migration.create_table_if_not_exists("StartTimestamps", |table| {
        table.add_column("component", types::text());
        table.add_column("timestamp", types::datetime());
    });

    migration.make::<Pg>()
}

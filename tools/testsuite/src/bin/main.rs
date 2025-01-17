/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

use std::env;
use std::process;
use std::sync::Arc;

use testsuite::config::Config;
use testsuite::flags::Flags;
use testsuite::tests::run_tests;

pub fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let flags = Flags::parse_from_args(&args);
    let config = Config::from_flags(flags);

    if !run_tests(Arc::new(config)) {
        process::exit(1);
    }
}

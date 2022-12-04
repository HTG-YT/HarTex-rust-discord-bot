/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

use crate::error::RepositoryResult;

pub trait Entity {
    type Id;

    fn id(&self) -> Self::Id;
}

pub trait Repository<T: Entity> {
    async fn get(&self, entity_id: T::Id) -> RepositoryResult<T>;

    async fn upsert(&self, entity: T) -> RepositoryResult<()>;
}

pub trait RepositoryUpdater {
    async fn update(&self) -> RepositoryResult<()>;
}
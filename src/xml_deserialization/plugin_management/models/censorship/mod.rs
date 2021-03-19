//!  Copyright 2020 - 2021 The HarTex Project Developers
//!
//!  Licensed under the Apache License, Version 2.0 (the "License");
//!  you may not use this file except in compliance with the License.
//!  You may obtain a copy of the License at
//!
//!      http://www.apache.org/licenses/LICENSE-2.0
//!
//!  Unless required by applicable law or agreed to in writing, software
//!  distributed under the License is distributed on an "AS IS" BASIS,
//!  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//!  See the License for the specific language governing permissions and
//!  limitations under the License.

extern crate serde;
extern crate quick_xml;

crate mod blacklisted_invite_codes;
crate mod domains_channel_whitelist;
crate mod invites_channel_whitelist;
crate mod whitelisted_domains;
crate mod whitelisted_guild_invites;
crate mod whitelisted_invite_codes;
crate mod zalgo_channel_whitelist;

#[derive(Debug, Clone, Serialize, Deserialize)]
crate struct CensoredUri {
    #[serde(rename = "Uri")]
    crate uri: String
}

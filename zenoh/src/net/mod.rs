//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
#[doc(hidden)]
pub use zenoh_link as link;
#[doc(hidden)]
pub mod protocol {
    #[deprecated = "This module is now a separate crate. Use the crate directly for shorter compile-times"]
    pub use zenoh_protocol::{io, proto};
    #[deprecated = "This module is now a separate crate. Use the crate directly for shorter compile-times"]
    pub use zenoh_protocol_core as core;
}
#[doc(hidden)]
pub mod routing;
#[doc(hidden)]
pub mod runtime;
#[doc(hidden)]
pub use zenoh_transport as transport;

// Copyright 2018-2024 the Deno authors. MIT license.

pub use joinset::JoinSet;
pub use split::split_io;
pub use split::IOReadHalf;
pub use split::IOWriteHalf;

pub use task::set_spawn_blocking_optional_use_current_thread;
pub use task::spawn;
pub use task::spawn_blocking_always;
pub use task::spawn_blocking_optional;
pub use task::JoinHandle;
pub use task::JoinResult;
pub use task::MaskFutureAsSend;

mod joinset;
mod split;
mod task;

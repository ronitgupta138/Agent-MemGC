pub mod cache_layout;
pub mod contradictions;
pub mod dead_paths;
pub mod redundancy;

pub use cache_layout::check_cache_layout;
pub use contradictions::check_contradictions;
pub use dead_paths::check_dead_paths;
pub use redundancy::check_redundancy;

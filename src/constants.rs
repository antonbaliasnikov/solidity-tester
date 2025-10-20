/// Default directory where repositories are cloned into
pub const DEFAULT_REPOS_DIR: &str = "repos";

// Default project manager if not provided in config or env
pub const DEFAULT_PROJECT_MANAGER: &str = "npm";

// Default ZKsync OS network name for Solidity projects
pub const NETWORK_NAME: &str = "ZKsyncOS";

// Default ZKsync OS chain ID for Solidity projects
pub const CHAIN_ID: &str = "270";

// Default ZKsync OS RPC URL for Solidity projects
pub const RPC_URL: &str = "http://localhost:3050";

// Default rich account private key for deploying contracts in tests
pub const RICH_ACCOUNT_PRIVATE_KEY: &str =
    "0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110";

// Default tmp account balance for each test
pub const DEFAULT_BALANCE: f64 = 0.5;

#[derive(Debug, Clone)]
pub enum PygoCommand {
    Run,
    Init { name: String, ptype: bool },
    New { name: String, ptype: bool },
    // Add(Vec<String>),
    // Clean
    // Test
}

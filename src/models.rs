
pub struct Command {
    pub signature: String,
    pub description: String,
    pub action_type: ActionType,
    pub technology: String,
    pub reference: String,
}

pub enum ActionType {
    Create,
    Delete,
    Help,
    Install,
    Read,
    Update,
    Permission
}

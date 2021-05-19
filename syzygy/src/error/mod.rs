pub trait Error: Send + Sync {}

pub struct RouterError {}

impl Error for RouterError {}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    goal: String,
    spec: String,
    children: Vec<PlanItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanItem {
    id: String,
    name: String,
    description: String,
    children: Vec<PlanItem>,
}

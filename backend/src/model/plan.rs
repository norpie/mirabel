use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    id: String,
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

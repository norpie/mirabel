import type { Plan, PlanItem } from "$lib/models/session";

export function getNodes(plan: Plan) {
    const nodes = [];
    const processNode = (item: PlanItem, parentId?: string) => {
        let type = 'default';
        if (item.children.length === 0) {
            type = 'output';
        }
        nodes.push({
            id: item.id,
            type,
            label: item.name,
            width: 150,
            height: 100
        });
        item.children.forEach(child => processNode(child, item.id));
    };
    nodes.push({ id: plan.id, label: plan.goal, type: 'input', width: 200, height: 60 });
    plan.children.forEach(child => processNode(child, plan.id));
    return nodes;
}

export function getEdges(plan: Plan) {
    const edges: {
        id: string;
        source: string;
        target: string;
        status: string;
    }[] = [];
    const processEdges = (item: PlanItem, parentId?: string) => {
        if (parentId) {
            edges.push({ id: `${parentId}->${item.id}`, source: parentId, target: item.id, status: item.status });
        }
        item.children.forEach(child => processEdges(child, item.id));
    };
    plan.children.forEach(child => processEdges(child, plan.id));
    return edges;
}

use crate::card::Card;

#[derive(PartialEq)]
enum UnoCondition {
    PlusStack,
}

struct DecisionNode<'a> {
    decision: &'a Card,
    branches: Vec<Branch<'a>>,
}

struct Branch<'a> {
    condition: UnoCondition,
    child: Box<Option<DecisionNode<'a>>>,
}

impl<'a> DecisionNode<'a> {
    pub fn follow(&'a self, conditions: &[UnoCondition]) -> Option<&'a Card> {
        let mut current_node = self;

        loop {
            // If node has no branches, it is a leaf and therefore a decision
            if current_node.branches.is_empty() {
                return Some(&current_node.decision);
            }

            let matched_branch = current_node
                .branches
                .iter()
                .find(|branch| conditions.contains(&branch.condition));

            match matched_branch {
                Some(branch) => {
                    // Move to the child node indicated by the matched branch
                    if let Some(child) = &*branch.child {
                        current_node = child;
                    } else {
                        // Child node is None, terminate traversal
                        return None;
                    }
                }
                None => {
                    // No branch matches the condition, terminate traversal
                    return None;
                }
            }
        }
    }
}

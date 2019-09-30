use std::sync::Arc;

#[derive(Debug)]
pub struct PathNode {
    parent: Option<Arc<Self>>,
    index: u16,
}

impl PathNode {
    pub fn new() -> Self {
        Self {
            parent: None,
            index: 0,
        }
    }

    pub fn derive(parent: Arc<Self>, index: u16) -> Self {
        Self {
            parent: Some(parent),
            index,
        }
    }
}

#[derive(Debug)]
pub struct Path {
    node: Arc<PathNode>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            node: Arc::new(PathNode::new()),
        }
    }

    pub fn derive(&self, index: u16) -> Self {
        Self {
            node: Arc::new(PathNode::derive(Arc::clone(&self.node), index)),
        }
    }
}


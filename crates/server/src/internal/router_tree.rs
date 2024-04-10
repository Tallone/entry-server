use axum::{
  handler::Handler,
  http::{Method, Request, Response, StatusCode},
  routing::{get, on, post, MethodFilter, MethodRouter},
  Router,
};
use std::{collections::HashMap, marker::PhantomData};

enum NodeType<H> {
  Leaf { handler: H, method: Method },
  Branch,
}

pub struct RouteNode<H, S> {
  path: String,
  node_type: NodeType<H>,
  children: HashMap<String, RouteNode<H, S>>,
  _s: PhantomData<S>,
}

impl<H, S> RouteNode<H, S>
where
  S: Clone + Send + Sync + 'static,
{
  pub fn new(path: &str) -> Self {
    RouteNode {
      path: path.to_string(),
      node_type: NodeType::Branch,
      children: HashMap::new(),
      _s: PhantomData,
    }
  }

  pub fn path(&mut self, path: &str) -> Self {
    let mut current_node = self;
    let mut parts = path.split('/').filter(|p| !p.is_empty());

    while let Some(part) = parts.next() {
      // Create a new branch node if it doesn't exist
      let child_node = current_node
        .children
        .entry(part.to_string())
        .or_insert_with(|| RouteNode::new(part));
      current_node = child_node;
    }
    *current_node
  }

  pub fn handler<T>(&mut self, handler: H, method: Method) -> &mut Self
  where
    H: Handler<T, S>,
    T: 'static,
  {
    let h = Self {
      node_type: NodeType::Leaf { handler, method },
      path: String::default(),
      children: HashMap::default(),
      _s: PhantomData,
    };
    self.children.insert(method.to_string(), h);
    self
  }

  pub fn nest(&mut self, other: Self) {
    let other_path = other.path.trim_start_matches('/');
    let mut parts = other_path.split('/').filter(|p| !p.is_empty());

    let mut current_node = self;
    while let Some(part) = parts.next() {
      let child_node = current_node
        .children
        .entry(part.to_string())
        .or_insert_with(|| RouteNode::new(part));
      current_node = child_node;

      if parts.next().is_none() {
        // This is the last part, merge the other node
        current_node.node_type = other.node_type;
        current_node.children = other.children;
      }
    }
  }

  pub fn to_axum_router<T>(&self) -> Router<S>
  where
    H: Handler<T, S>,
    T: 'static,
  {
    let mut router = Router::new();
    self.add_to_axum_router(&mut router);
    router
  }

  fn add_to_axum_router<T>(&self, router: &mut Router<S>)
  where
    H: Handler<T, S>,
    T: 'static,
  {
    if let NodeType::Leaf { handler, method } = &self.node_type {
      match method {
        &Method::GET => router.route(&self.path, on(MethodFilter::GET, *handler)),
        &Method::POST => router.route(&self.path, on(MethodFilter::POST, *handler)),
        _ => panic!("Unsupported HTTP method: {}", method),
      };
    } else {
      for child in self.children.values() {
        child.add_to_axum_router(router);
      }
    };
  }
}

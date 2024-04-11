use axum::{http::Method, routing::MethodRouter, Router};
use log::info;
use prettytable::{
  format::{FormatBuilder, LinePosition, LineSeparator},
  row, Cell, Row, Table,
};

use super::app_state::AppState;

// Defines the different types of nodes in the routing tree
enum NodeType<H> {
  // Leaf node represents a specific route handler
  Leaf(Method, H),
  // Branch node represents a branch in the routing tree
  Branch,
}

// Represents a node in the routing tree
pub struct RouteNode {
  // The path associated with this node
  path: String,
  // The type of this node (Leaf or Branch)
  node_type: NodeType<MethodRouter<AppState>>,
  // The child nodes of this node
  children: Vec<RouteNode>,
}

impl RouteNode {
  // Creates a new branch node with the given path
  pub fn new(path: &str) -> Self {
    RouteNode {
      path: path.to_string(),
      node_type: NodeType::Branch,
      children: Vec::new(),
    }
  }
  // Adds a new path segment to the current node, creating new branch nodes as needed.
  // This function will return latest segment as node
  pub fn path(&mut self, path: &str) -> &mut Self {
    let mut current_node = self;
    let parts = path.split('/').filter(|p| !p.is_empty());

    for part in parts {
      // Create a new branch node if it doesn't exist
      let child_node = Self::new(part);
      current_node.children.push(child_node);
      current_node = current_node.children.last_mut().unwrap();
    }
    current_node
  }

  // Adds a new leaf node with the given method and handler to the current node
  pub fn handler(&mut self, method: Method, handler: MethodRouter<AppState>) -> &mut Self {
    let h = Self {
      node_type: NodeType::Leaf(method, handler),
      path: String::default(),
      children: Vec::new(),
    };
    self.children.push(h);
    self
  }

  // Nests another RouteNode under the current node
  pub fn nest(&mut self, path: &str, mut other: Self) -> &mut Self {
    let parent = self.path(path);
    other.path = other.path.trim_start_matches('/').to_owned();
    parent.children.push(other);
    self
  }

  // Converts the current node and its children into an Axum router
  #[allow(clippy::wrong_self_convention)]
  pub fn to_axum_router(self) -> Router<AppState> {
    let router = Router::new();
    let mut path = self.path.clone();

    let mut table = Table::new();
    // This format can directly used in markdown
    let format = FormatBuilder::new()
      .padding(1, 1)
      .separators(&[LinePosition::Title], LineSeparator::new('-', '|', '-', '-'))
      .column_separator('|')
      .borders('|')
      .build();
    table.set_format(format);
    table.set_titles(row!["Method", "Endpoint"]);

    let ret = Self::add_to_axum_router(self, &mut table, &mut path, router);
    info!("\n{}", table.to_string());
    ret
  }

  fn add_to_axum_router(
    node: RouteNode,
    table: &mut Table,
    path: &mut String,
    mut router: Router<AppState>,
  ) -> Router<AppState> {
    if let NodeType::Leaf(m, h) = node.node_type {
      table.add_row(Row::new(vec![Cell::new(m.as_ref()), Cell::new(path)]));
      return router.route(path, h);
    }

    for child in node.children {
      if let NodeType::Branch = child.node_type {
        if child.path.is_empty() {
          router = Self::add_to_axum_router(child, table, path, router);
        } else {
          path.push('/');
          path.push_str(&child.path);
          let l = child.path.len() + 1;
          router = Self::add_to_axum_router(child, table, path, router);
          path.truncate(path.len() - l);
        }
      } else {
        router = Self::add_to_axum_router(child, table, path, router);
      }
    }

    router
  }
}

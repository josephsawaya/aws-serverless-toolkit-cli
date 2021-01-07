use crate::helper;
use colored::*;
use std::fmt;
use yaml_rust::Yaml;

pub enum BillingMode {
  PayPerRequest,
  Provisioned {
    read_cap: Option<usize>,
    write_cap: Option<usize>,
  },
}

impl fmt::Display for BillingMode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      BillingMode::PayPerRequest => write!(f, "PayPerRequest"),
      BillingMode::Provisioned {
        read_cap: _,
        write_cap: _,
      } => write!(f, "Provisioned"),
    }
  }
}

enum RemovalPolicy {
  Destroy,
  Retain,
  Snapshot,
}

impl Copy for BillingMode {}

impl Clone for BillingMode {
  fn clone(&self) -> BillingMode {
    *self
  }
}

impl fmt::Display for RemovalPolicy {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      RemovalPolicy::Destroy => write!(f, "Destroy"),
      RemovalPolicy::Retain => write!(f, "Retain"),
      RemovalPolicy::Snapshot => write!(f, "Snapshot"),
    }
  }
}

enum KeyType {
  S,
  N,
  B,
}

impl fmt::Display for KeyType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      KeyType::S => write!(f, "S"),
      KeyType::N => write!(f, "N"),
      KeyType::B => write!(f, "B"),
    }
  }
}

pub struct Table {
  table_name: String,
  pub billing_mode: BillingMode,
  pk_name: String,
  pk_type: KeyType,
  sk_name: Option<String>,
  sk_type: Option<KeyType>,
  removal_policy: RemovalPolicy,
}
impl Table {
  pub fn new() -> Table {
    Table {
      table_name: String::from(""),
      billing_mode: BillingMode::PayPerRequest,
      pk_name: String::from(""),
      pk_type: KeyType::S,
      sk_name: None,
      sk_type: None,
      removal_policy: RemovalPolicy::Destroy,
    }
  }

  pub fn set_table_name(&mut self, table_name: String, config: &Yaml) {
    let res = helper::fetch_names_from_config(config, String::from("Table"));
    if res.contains(&table_name) {
      println!(
        "{} {}",
        "error: ".bright_red().bold(),
        "a table with that name already exists"
      );
      std::process::exit(0);
    }
    self.table_name = table_name;
  }

  pub fn set_billing_mode(&mut self, billing_mode: usize) {
    match billing_mode {
      0 => self.billing_mode = BillingMode::PayPerRequest,
      1 => {
        self.billing_mode = BillingMode::Provisioned {
          read_cap: None,
          write_cap: None,
        }
      }
      _ => (),
    }
  }

  pub fn set_read_cap(&mut self, cap: usize) {
    if let BillingMode::Provisioned {
      read_cap: _,
      write_cap,
    } = self.billing_mode
    {
      self.billing_mode = BillingMode::Provisioned {
        read_cap: Some(cap),
        write_cap,
      }
    }
  }

  pub fn set_write_cap(&mut self, cap: usize) {
    if let BillingMode::Provisioned {
      read_cap,
      write_cap: _,
    } = self.billing_mode
    {
      self.billing_mode = BillingMode::Provisioned {
        read_cap,
        write_cap: Some(cap),
      }
    }
  }

  pub fn set_pk_name(&mut self, name: String) {
    self.pk_name = name;
  }

  pub fn set_pk_type(&mut self, new_type: usize) {
    match new_type {
      0 => self.pk_type = KeyType::S,
      1 => self.pk_type = KeyType::N,
      2 => self.pk_type = KeyType::B,
      _ => (),
    }
  }

  pub fn set_sk_name(&mut self, name: String) {
    self.sk_name = Some(name);
  }

  pub fn set_sk_type(&mut self, new_type: usize) {
    match new_type {
      0 => self.sk_type = Some(KeyType::S),
      1 => self.sk_type = Some(KeyType::N),
      2 => self.sk_type = Some(KeyType::B),
      _ => (),
    }
  }

  pub fn set_removal_policy(&mut self, removal_policy: usize) {
    match removal_policy {
      0 => self.removal_policy = RemovalPolicy::Destroy,
      1 => self.removal_policy = RemovalPolicy::Retain,
      2 => self.removal_policy = RemovalPolicy::Snapshot,
      _ => (),
    }
  }

  pub fn create_string(&self) -> String {
    format!(
      "   - Table:
      tableName: {}
      billing:
          mode: {}{}
      partitionKey:
          name: {}
          type: {}{}
      removalPolicy: {}
",
      self.table_name,
      self.billing_mode,
      if let BillingMode::Provisioned {
        read_cap,
        write_cap,
      } = self.billing_mode
      {
        format!(
          "
      readCapactity: {}
      writeCapactity: {}",
          read_cap.unwrap(),
          write_cap.unwrap()
        )
      } else {
        String::from("")
      },
      self.pk_name,
      self.pk_type,
      if let None = self.sk_name {
        String::from("")
      } else {
        format!(
          "
      sortKey:
          name: {}
          type: {}",
          self.sk_name.as_ref().unwrap(),
          self.sk_type.as_ref().unwrap()
        )
      },
      self.removal_policy,
    )
  }
}

pub enum Method {
  POST,
  GET,
}
pub struct Lambda {
  function_name: String,
  table_list: std::vec::Vec<String>,
  method: Method,
  path: String,
}

impl Lambda {
  pub fn new() -> Lambda {
    Lambda {
      function_name: String::from(""),
      method: Method::GET,
      path: String::from(""),
      table_list: std::vec::Vec::new(),
    }
  }

  pub fn set_function_name(&mut self, name: String, config: &Yaml) {
    let res = helper::fetch_names_from_config(config, String::from("Function"));
    if res.contains(&name) {
      println!(
        "{} {}",
        "error: ".bright_red().bold(),
        "a table with that name already exists"
      );
      std::process::exit(0);
    }
    self.function_name = name;
  }
  pub fn set_table_list(&mut self, table_list: std::vec::Vec<usize>, config: &Yaml) {
    let table_names = crate::helper::fetch_names_from_config(config, String::from("Table"));
    let mut res: std::vec::Vec<String> = std::vec::Vec::new();
    for x in table_list {
      res.push(String::clone(&table_names[x]));
    }
    self.table_list = res;
  }

  pub fn set_method(&mut self, method: usize) {
    match method {
      0 => self.method = Method::GET,
      1 => self.method = Method::POST,
      _ => (),
    }
  }

  pub fn set_path(&mut self, path: String, config: &Yaml) {
    let config = config.as_hash().unwrap();
    for (_, val) in config {
      for elem in val.as_vec().unwrap() {
        let b = elem.as_hash().unwrap();
        let function_key = &Yaml::String(String::from("Function"));
        if b.contains_key(function_key) {
          let path_key = &Yaml::String(String::from("path"));
          if b[function_key].as_hash().unwrap()[path_key]
            .as_str()
            .unwrap()
            == path
          {
            println!(
              "{} {}",
              "error: ".bright_red().bold(),
              "a function with that path already exists"
            );
            std::process::exit(0);
          }
        }
      }
    }
    self.path = path;
  }

  pub fn create_string(&self) -> String {
    let mut table_list_string = String::new();
    for x in &self.table_list {
      table_list_string = format!(
        "{}
      - {}",
        table_list_string, x
      );
    }
    format!(
      "   - Function:
      functionName: {}
      method: {}
      path: {}
      tableList: {}
",
      self.function_name,
      if let Method::GET = self.method {
        String::from("GET")
      } else {
        String::from("POST")
      },
      self.path,
      table_list_string
    )
  }
}

pub struct Api {
  api_name: String,
  function_list: std::vec::Vec<String>,
}

impl Api {
  pub fn new() -> Api {
    Api {
      api_name: String::new(),
      function_list: std::vec::Vec::new(),
    }
  }

  pub fn set_api_name(&mut self, name: String, config: &Yaml) {
    let res = helper::fetch_names_from_config(config, String::from("Api"));
    if res.contains(&name) {
      println!(
        "{} {}",
        "error: ".bright_red().bold(),
        "a table with that name already exists"
      );
      std::process::exit(0);
    }
    self.api_name = name;
  }

  pub fn set_function_list(&mut self, function_list: std::vec::Vec<usize>, config: &Yaml) {
    let function_names = crate::helper::fetch_names_from_config(config, String::from("Function"));
    let mut res: std::vec::Vec<String> = std::vec::Vec::new();
    for x in function_list {
      res.push(String::clone(&function_names[x]));
    }
    self.function_list = res;
  }

  pub fn create_string(&self) -> String {
    let mut function_list_string = String::new();
    for x in &self.function_list {
      function_list_string = format!(
        "{}
      - {}",
        function_list_string, x
      );
    }
    format!(
      "   - Api:
      apiName: {}
      functionList: {}
",
      self.api_name, function_list_string
    )
  }
}

// attach table to function
// attach function to api

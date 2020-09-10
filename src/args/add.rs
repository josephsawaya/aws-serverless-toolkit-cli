pub fn add(added: &String) {
  crate::helper::check_project_integrity();
  let mut file_contents = match crate::helper::fetch_or_create_ast_config() {
    Ok(ast_config_contents) => ast_config_contents,
    Err(_) => {
      std::fs::write("./ast.yaml", "").expect("trouble writing to config file");
      String::new()
    }
  };
  match &added[..] {
    "table" => {
      let table_name = dialoguer::Input::<String>::new()
        .with_prompt("Table Name")
        .allow_empty(false)
        .interact()
        .expect("Expected an argument");

      let billing_mode_options = vec!["PAY_PER_REQUEST", "PROVISIONED"];
      let billing_mode_index = dialoguer::Select::new()
        .with_prompt("Billing Mode")
        .default(0)
        .items(&billing_mode_options)
        .interact()
        .expect("Error choosing billing mode");

      let partition_key_name = dialoguer::Input::<String>::new()
        .with_prompt("Partition Key Name")
        .allow_empty(false)
        .interact()
        .expect("Expected an argument");
      let key_type_options = vec!["STRING", "NUMBER", "BINARY"];
      let partition_key_type_index = dialoguer::Select::new()
        .with_prompt("Partition Key Type")
        .default(0)
        .items(&key_type_options)
        .interact()
        .expect("Error choosing partition key type");

      let yes_or_no = vec!["Yes", "No"];
      let sort_key_yes_or_no = dialoguer::Select::new()
        .with_prompt("Do you want this table to have a sort key?")
        .default(0)
        .items(&yes_or_no)
        .interact()
        .expect("Error choosing whether this table should have a sort key or not");
      let mut sort_key_name: Option<String> = None;
      let mut sort_key_type: Option<&str> = None;
      if yes_or_no[sort_key_yes_or_no] == "Yes" {
        sort_key_name = match dialoguer::Input::<String>::new()
          .with_prompt("Sort Key Name")
          .allow_empty(false)
          .interact()
          .expect("Expected an argument")
        {
          name => Some(name),
        };
        sort_key_type = match dialoguer::Select::new()
          .with_prompt("Sort Key Type")
          .default(0)
          .items(&key_type_options)
          .interact()
          .expect("Error choosing sort key type")
        {
          temp => Some(key_type_options[temp]),
        };
      }
      let read_capacity = match billing_mode_index {
        1 => Some(
          dialoguer::Input::<usize>::new()
            .with_prompt("Read capacity")
            .allow_empty(false)
            .interact()
            .expect("Expected an argument"),
        ),
        _ => None,
      };
      let write_capacity = match billing_mode_index {
        1 => Some(
          dialoguer::Input::<usize>::new()
            .with_prompt("Write capacity")
            .allow_empty(false)
            .interact()
            .expect("Expected an argument"),
        ),
        _ => None,
      };
      let removal_policy_options = vec!["DESTROY", "RETAIN", "SNAPSHOT"];
      let removal_policy_index = dialoguer::Select::new()
        .with_prompt("Removal Policy")
        .default(0)
        .items(&removal_policy_options)
        .interact()
        .expect("Error choosing removal policy");
      file_contents = format!(
        "{}{}:\n\tbillingMode: {}\n\tpartitionKey:\n\t\tname: {}\n\t\ttype: {}\n",
        file_contents,
        table_name,
        &billing_mode_options[billing_mode_index],
        partition_key_name,
        &key_type_options[partition_key_type_index]
      );
      file_contents = match sort_key_name {
        Some(name) => format!("{}\tsortKey:\n\t\tname: {}\n", file_contents, name),
        None => file_contents,
      };
      file_contents = match sort_key_type {
        Some(temp) => format!("{}\t\ttype: {}\n", file_contents, temp),
        None => file_contents,
      };
      file_contents = match read_capacity {
        Some(temp) => format!("{}\treadCapacity: {}\n", file_contents, temp),
        None => file_contents,
      };
      file_contents = match write_capacity {
        Some(temp) => format!("{}\twriteCapacity: {}\n", file_contents, temp),
        None => file_contents,
      };
      file_contents = format!(
        "{}\tremovalPolicy: {}\n",
        file_contents, removal_policy_options[removal_policy_index]
      );
    }
    "lambda" => file_contents.push_str("lambda:\n"),
    "apigateway" => file_contents.push_str("apigateway:\n"),
    _ => (),
  }
  std::fs::write("./ast.yaml", file_contents).expect("Trouble writing to config file");
}

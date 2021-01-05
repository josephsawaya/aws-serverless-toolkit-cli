pub fn add(added: Option<String>) {
  use crate::types;
  use colored::*;
  use std::fs::File;
  use std::io::prelude::*;
  use yaml_rust::YamlLoader;
  crate::helper::check_project_integrity();
  let mut file_contents = match crate::helper::fetch_or_create_ast_config() {
    Ok(ast_config_contents) => ast_config_contents,
    Err(_) => {
      std::fs::write("./ast.yaml", "Config:\n").expect("trouble writing to config file");
      String::from("Config:\n")
    }
  };
  let mut file = File::open("./ast.yaml").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  println!("{:?}", contents);
  let config = YamlLoader::load_from_str(&mut contents).unwrap();
  match added.as_deref() {
    Some("table") => {
      let mut new_table = types::Table::new();
      new_table.set_table_name(
        dialoguer::Input::new()
          .with_prompt("Table Name")
          .allow_empty(false)
          .interact()
          .unwrap(),
        &config[0],
      );
      let billing_mode_options = vec!["PAY_PER_REQUEST", "PROVISIONED"];
      new_table.set_billing_mode(
        dialoguer::Select::new()
          .with_prompt("Billing Mode")
          .default(0)
          .items(&billing_mode_options)
          .interact()
          .unwrap(),
      );
      if let types::BillingMode::Provisioned {
        read_cap: _,
        write_cap: _,
      } = &new_table.billing_mode
      {
        new_table.set_read_cap(
          dialoguer::Input::<usize>::new()
            .with_prompt("Read capacity")
            .allow_empty(false)
            .interact()
            .unwrap(),
        );
        new_table.set_write_cap(
          dialoguer::Input::<usize>::new()
            .with_prompt("Write capacity")
            .allow_empty(false)
            .interact()
            .unwrap(),
        );
      }

      new_table.set_pk_name(
        dialoguer::Input::<String>::new()
          .with_prompt("Partition Key Name")
          .allow_empty(false)
          .interact()
          .unwrap(),
      );
      let key_type_options = vec!["STRING", "NUMBER", "BINARY"];
      new_table.set_pk_type(
        dialoguer::Select::new()
          .with_prompt("Partition Key Type")
          .default(0)
          .items(&key_type_options)
          .interact()
          .unwrap(),
      );

      let yes_or_no = vec!["Yes", "No"];
      let sort_key_yes_or_no = dialoguer::Select::new()
        .with_prompt("Do you want this table to have a sort key?")
        .default(0)
        .items(&yes_or_no)
        .interact()
        .unwrap();
      if sort_key_yes_or_no == 0 {
        new_table.set_sk_name(
          dialoguer::Input::<String>::new()
            .with_prompt("Sort Key Name")
            .allow_empty(false)
            .interact()
            .unwrap(),
        );
        new_table.set_sk_type(
          dialoguer::Select::new()
            .with_prompt("Sort Key Type")
            .default(0)
            .items(&key_type_options)
            .interact()
            .unwrap(),
        );
      }
      let removal_policy_options = vec!["DESTROY", "RETAIN", "SNAPSHOT"];
      new_table.set_removal_policy(
        dialoguer::Select::new()
          .with_prompt("Removal Policy")
          .default(0)
          .items(&removal_policy_options)
          .interact()
          .unwrap(),
      );
      file_contents = format!("{}{}", file_contents, new_table.create_string());
      print!("{}", "Writing...".bright_green().bold());
      std::fs::write("./ast.yaml", file_contents).expect("Trouble writing to config file");
      print!(
        "{}",
        "\rChanges written to ast.yaml\n".bright_green().bold()
      );
    }
    Some("lambda") => {}
    Some("api") => file_contents.push_str("api:\n"),
    Some(_) => print!(
      "{} {} {} {}",
      "error:".bright_red().bold(),
      "Invalid Argument, run",
      "ast-cli add".bold(),
      "to see possible commands\n"
    ),
    None => print!(
      "ast-cli add
    adding a component to our ast-serverless-toolkit project config
        
    USAGE:
        ast-cli [COMMAND]
            
        Commands are:
        table           Add a DynamoDB table
        lambda          Add a Lambda function
        api             Add an ApiGateway REST Api\n"
    ),
  }
}

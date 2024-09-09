use anyhow::anyhow;
use regex::Regex;
use semver::{Version, VersionReq};
use std::process::Command;
use toml_edit::{value, DocumentMut, Item, Table};

fn is_compatible(actual_version: &Version, version_requirement: &str) -> Option<bool> {
    VersionReq::parse(version_requirement)
        .and_then(|x| Ok(x.matches(&actual_version)))
        .ok()
}

fn parse_semver(tool_name: &str, stdout: &str) -> Option<String> {
    let re = Regex::new(&format!(r"{} (\d+\.\d+\.\d+)", tool_name)).unwrap();
    re.captures(stdout).and_then(|x| Some(x[1].to_string()))
}

pub fn get_tool_version(tool_name: &str) -> anyhow::Result<Version> {
    let mut command = Command::new(tool_name);
    let uv_v = command.arg("-V");
    let out = uv_v
        .output()
        .expect(&format!("Cannot run '{} -V' command", tool_name));
    let out_string =
        String::from_utf8(out.stdout).expect(&format!("Cannot fetch '{} -V' output", tool_name));
    let out_semver =
        parse_semver(tool_name, &out_string).ok_or(anyhow!("Cannot parse version number"))?;
    Version::parse(&out_semver).map_err(anyhow::Error::from)
}

fn nest(item: &mut Item, old_key: &str, new_key: &str) {
    if let Some((key, universal)) = item
        .as_table_mut()
        .expect("Error while parsing the item as as table")
        .remove_entry(old_key)
    {
        let mut sub_table = item
            .get_mut(new_key)
            .and_then(|x| x.as_table_mut())
            .unwrap_or(&mut Table::new())
            .clone();

        sub_table.insert(&key, universal);
        item[new_key] = Item::Table(sub_table);
    }
}

fn rename(table: &mut Table, key: &str, new_key: &str) {
    if let Some(value) = table.remove(key) {
        table[new_key] = value;
    }
}

fn rename_and_invert(table: &mut Item, key: &str, new_key: &str) -> anyhow::Result<()> {
    if let Some(key_val) = table.as_table_mut().and_then(|x| x.remove(key)) {
        let new_val = key_val.as_bool();
        table[new_key] = value(!new_val.ok_or(anyhow!("Value of {} is not a valid bool", key))?)
    }
    Ok(())
}

pub fn convert(document: &mut DocumentMut, uv: Version) -> anyhow::Result<()> {
    let uv_ge_4 = is_compatible(&uv, ">=0.4.0");

    if let Some(tool) = document.get_mut("tool") {
        if let Some(tool) = tool.as_table_mut() {
            rename(tool, "rye", "uv");

            nest(&mut tool["uv"], "universal", "pip");
            nest(&mut tool["uv"], "generate-hashes", "pip");

            rename_and_invert(&mut tool["uv"], "lock-with-sources", "no-sources")?;

            if let Some(uv_4_compatible) = uv_ge_4 {
                if uv_4_compatible {
                    rename_and_invert(&mut tool["uv"], "virtual", "package")?;
                } else {
                    tool["uv"].as_table_mut().and_then(|x| x.remove("virtual"));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    use toml_edit::Value;

    use super::*;

    #[test]
    fn test_rename() -> anyhow::Result<()> {
        let mut table = Table::new();
        let val = Item::Value(Value::from_str(&"'foo'")?);
        table["test_foo"] = val.clone();
        rename(&mut table, "test_foo", "test_bar");
        assert!(table.contains_key("test_bar"));
        assert!(!table.contains_key("test_foo"));
        assert_eq!(table["test_bar"].to_string(), val.to_string());
        Ok(())
    }

    #[test]
    fn test_rename_and_invert() -> anyhow::Result<()> {
        let mut table = Table::new();
        let mut inner_table = Table::new();
        let val = Item::Value(Value::from(true));
        inner_table["test_foo"] = val.clone();
        table["bool_val"] = Item::Table(inner_table);

        rename_and_invert(&mut table["bool_val"], "test_foo", "test_bar")?;
        assert_eq!(table["bool_val"]["test_bar"].as_bool().unwrap(), false);
        Ok(())
    }
}

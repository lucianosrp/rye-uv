#![allow(unused)]
use std::{
    error::Error,
    fs::{self},
    io::{Read, Write},
};

use toml_edit::{value, DocumentMut, Item, Table};

/// nest: [item.old_key] => [item.new_key.old_key]
pub fn nest(item: &mut Item, old_key: &str, new_key: &str) {
    if let Some((key, universal)) = item
        .as_table_mut()
        .expect("Cannot find the tool.uv")
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

pub fn rename(table: &mut Table, key: &str, new_key: &str) {
    if let Some(value) = table.remove(key) {
        table[new_key] = value;
    }
}

pub fn convert(document: &mut DocumentMut) -> anyhow::Result<()> {
    if let Some(tool) = document.get_mut("tool") {
        if let Some(tool) = tool.as_table_mut() {
            rename(tool, "rye", "uv");

            nest(&mut tool["uv"], "universal", "pip");
            nest(&mut tool["uv"], "generate-hashes", "pip");

            // (lock-with-sources = false) => (no-sources = true)
            if let Some(lock_with_sources) = tool["uv"]
                .as_table_mut()
                .and_then(|x| x.remove("lock-with-sources"))
            {
                tool["uv"]["no-sources"] = value(!lock_with_sources.as_bool().unwrap());
            }

            tool["uv"].as_table_mut().and_then(|x| x.remove("virtual"));
        }
    }

    Ok(())
}

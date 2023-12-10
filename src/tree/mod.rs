use anyhow::{Ok, Result};

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Tree {
    object: Object,
}

impl Tree {
    pub fn print(&self) {
        let mut all = self
            .object
            .content
            .split(|b| *b == 0)
            .map(|c| {
                String::from_utf8(
                    c.split(|b| *b == 32)
                        .collect::<Vec<&[u8]>>()
                        .last()
                        .expect("Wrong tree object format")
                        .to_vec(),
                )
                .unwrap_or_default()
            })
            .filter(|c| !c.is_empty())
            .collect::<Vec<String>>();
        all.remove(0);
        println!("{}", all.join("\n"));
    }

    pub fn read(complete_checksum: String) -> Result<Self> {
        return Ok(Tree {
            object: Object::read(complete_checksum)?,
        });
    }

}

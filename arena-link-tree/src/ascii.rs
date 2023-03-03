use std::fmt::{self, Write};

use crate::{NodeId, Tree};

impl<T: Default> Tree<T> {
    pub fn ascii(&self, data_fmt: &impl Fn(&T) -> String) -> String {
        if self.is_empty() {
            "".to_string()
        } else {
            self.ascii_node(self.root(), data_fmt)
        }
    }
    pub fn ascii_node(&self, id: NodeId, data_fmt: &impl Fn(&T) -> String) -> String {
        let mut s = String::new();
        self.write_elem(&mut s, id, &vec![], data_fmt).unwrap();
        s
    }

    fn siblings(&self, mut id: NodeId) -> Vec<NodeId> {
        let mut children = vec![id];
        while let Some(next_sibling) = self.nodes[id.index()].prev_sibling {
            children.push(next_sibling);
            id = next_sibling;
        }
        children
    }

    fn write_elem(
        &self,
        f: &mut dyn Write,
        id: NodeId,
        level: &Vec<usize>,
        data_fmt: &impl Fn(&T) -> String,
    ) -> fmt::Result {
        const EMPTY: &str = "    ";
        const EDGE: &str = " └─";
        const PIPE: &str = " │  ";
        const BRANCH: &str = " ├─";

        let maxpos = level.len();
        let mut second_line = String::new();
        for (pos, l) in level.iter().enumerate() {
            let last_row = pos == maxpos - 1;
            if *l == 1 {
                if !last_row {
                    write!(f, "{}", EMPTY)?
                } else {
                    write!(f, "{}", EDGE)?
                }
                second_line.push_str(EMPTY);
            } else {
                if !last_row {
                    write!(f, "{}", PIPE)?
                } else {
                    write!(f, "{}", BRANCH)?
                }
                second_line.push_str(PIPE);
            }
        }
        let title = (*data_fmt)(&self.nodes[id.index()].data);
        match self.nodes[id.index()].last_child {
            Some(child) => {
                let mut children = self.siblings(child);
                children.reverse();
                let mut d = children.len();
                if level.len() == 0 {
                    write!(f, "{}\n", title)?;
                } else {
                    write!(f, " {}\n", title)?;
                }

                for s in children {
                    let mut lnext = level.clone();
                    lnext.push(d);
                    d -= 1;
                    self.write_elem(f, s, &lnext, data_fmt)?;
                }
            }
            None => writeln!(f, " {}", title)?,
        }
        Ok(())
    }
}

use swayipc::{Connection, Workspace};

// newtype wrapper for sway workspaces
pub struct Workspaces(Vec<Workspace>);

impl Workspaces {
    pub fn get(connection: &mut Connection) -> Result<Self, swayipc::Error> {
        Ok(Workspaces(connection.get_workspaces()?))
    }

    pub fn ordered(self, connection: &mut Connection) -> Result<OrderedWorkspaces, swayipc::Error> {
        self.reorder(connection)?;
        let names = self.names();
        let (current_index, current_workspace) = self.current_workspace();

        Ok(OrderedWorkspaces {
            names,
            current_index,
            current_empty: current_workspace.representation.is_none(),
        })
    }

    pub fn reorder(&self, connection: &mut Connection) -> Result<(), swayipc::Error> {
        // make sure that workspace numbers are correctly ordered

        let workspaces = &self.0;

        // renumbering down
        for (index, workspace) in workspaces.iter().enumerate() {
            // 1-indexing
            let index = index + 1;

            // workspace number can be -1 if it's not numbered
            let num: Option<usize> = workspace.num.try_into().ok();
            let name = workspace.name.trim_start_matches(char::is_numeric);

            if let Some(num) = num {
                if num > index {
                    connection.run_command(format!(
                        "rename workspace \"{num}{name}\" to \"{index}{name}\""
                    ))?;
                }
            } else {
                connection
                    .run_command(format!("rename workspace \"{name}\" to \"{index}{name}\""))?;
            }
        }

        // renumbering up
        for (index, workspace) in workspaces.iter().enumerate().rev() {
            // 1-indexing
            let index = index + 1;

            // workspace number can be -1 if it's not numbered
            let num: Option<usize> = workspace.num.try_into().ok();
            let name = workspace.name.trim_start_matches(char::is_numeric);

            // only consider numbered workspaces
            if let Some(num) = num {
                if num < index {
                    connection.run_command(format!(
                        "rename workspace \"{num}{name}\" to \"{index}{name}\""
                    ))?;
                }
            }
        }

        Ok(())
    }

    fn names(&self) -> Vec<Option<String>> {
        // 1-indexed list of workspace names
        std::iter::once(None)
            .chain(self.0.iter().map(|workspace| {
                Some(
                    workspace
                        .name
                        .trim_start_matches(char::is_numeric)
                        .to_string(),
                )
            }))
            .collect()
    }

    pub fn current_workspace(&self) -> (usize, &Workspace) {
        let (index, workspace) = self
            .0
            .iter()
            .enumerate()
            .find(|(_, workspace)| workspace.focused)
            .expect("current workspace not found");
        // 1-indexing
        (index + 1, workspace)
    }
}

pub struct OrderedWorkspaces {
    names: Vec<Option<String>>,
    current_index: usize,
    current_empty: bool,
}

impl OrderedWorkspaces {
    pub fn names(&self) -> &Vec<Option<String>> {
        &self.names
    }

    pub fn name(&self, index: usize) -> &str {
        // this does not check for workspace bounds
        if index < self.names.len() {
            self.names[index].as_ref().unwrap()
        } else {
            ""
        }
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn current_empty(&self) -> bool {
        self.current_empty
    }

    pub fn insert(
        &mut self,
        connection: &mut Connection,
        index: usize,
    ) -> Result<(), swayipc::Error> {
        for i in (index..self.names.len()).rev() {
            if let Some(name) = &self.names[i] {
                connection.run_command(format!(
                    "rename workspace \"{i}{name}\" to \"{j}{name}\"",
                    j = i + 1
                ))?;
            }
        }
        self.names.insert(index, None);
        Ok(())
    }
}

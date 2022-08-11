pub enum Position {
    Prev { cycle: bool },
    Next { cycle: bool },
    Start,
    End,
    Num(usize),
}

impl Position {
    pub fn num_existing(
        &self,
        current_index: usize,
        num_workspaces: usize,
    ) -> Result<usize, swayipc::Error> {
        let (index, len) = (current_index, num_workspaces);

        match *self {
            Position::Prev { cycle } => {
                if index == 0 {
                    if cycle {
                        Ok(len - 1)
                    } else {
                        Err(swayipc::Error::CommandFailed(
                            "No previous workspace in the first workspace".to_string(),
                        ))
                    }
                } else {
                    Ok(index - 1)
                }
            }

            Position::Next { cycle } => {
                if index == len - 1 {
                    if cycle {
                        Ok(0)
                    } else {
                        Err(swayipc::Error::CommandFailed(
                            "No next workspace in the last workspace".to_string(),
                        ))
                    }
                } else {
                    Ok(index + 1)
                }
            }

            Position::Start => Ok(0),

            Position::End => Ok(len - 1),

            Position::Num(num) => {
                if num <= len {
                    Ok(num)
                } else {
                    Err(swayipc::Error::CommandFailed(
                        "Workspace number out of range".to_string(),
                    ))
                }
            }
        }
    }

    pub fn num_new(
        &self,
        current_index: usize,
        num_workspaces: usize,
    ) -> Result<usize, swayipc::Error> {
        let (index, len) = (current_index, num_workspaces);

        match *self {
            Position::Prev { cycle: _ } => Ok(index),

            Position::Next { cycle: _ } => Ok(index + 1),

            Position::Start => Ok(0),

            Position::End => Ok(len),

            Position::Num(num) => {
                if num <= len {
                    Ok(num)
                } else {
                    Err(swayipc::Error::CommandFailed(
                        "Workspace number out of range".to_string(),
                    ))
                }
            }
        }
    }
}

pub enum Command {
    Switch { target: Position, carry: bool },
    Create { target: Position, carry: bool },
    Swap { target: Position },
}

impl Command {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let verb = args.next().ok_or("not enough arguments")?;
        let position = args.next().ok_or("not enough arguments")?;
        let cycle = match args.next() {
            Some(option) => option.as_str() == "--cycle",
            None => false,
        };

        let target = match position.as_str() {
            "prev" => Ok(Position::Prev { cycle }),
            "next" => Ok(Position::Next { cycle }),
            "start" => Ok(Position::Start),
            "end" => Ok(Position::End),
            other => other
                .parse::<usize>()
                .map(Position::Num)
                .or(Err("invalid target")),
        }?;

        match verb.as_str() {
            "switch" => Ok(Self::Switch {
                target,
                carry: false,
            }),
            "move" => Ok(Self::Switch {
                target,
                carry: true,
            }),
            "create" => Ok(Self::Create {
                target,
                carry: false,
            }),
            "move-to-new" => Ok(Self::Create {
                target,
                carry: true,
            }),
            "swap" => Ok(Self::Swap { target }),

            _ => Err("invalid commnd"),
        }
    }
}

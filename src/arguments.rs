use std::str::FromStr;

use anyhow::bail;

#[derive(Clone, Copy, Debug)]
pub enum CommandArgument {
    Dance,
    Wait,
    BotId(usize),
    Go,
}

impl FromStr for CommandArgument {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<CommandArgument, Self::Err> {
        use CommandArgument::*;
        let out = if let Ok(bot_id) = s.parse::<usize>() {
            BotId(bot_id)
        } else {
            match s {
                "dance" => Dance,
                "wait" => Wait,
                "go" => Go,
                _ => bail!("Unrecognized argument: `{}`", s),
            }
        };
        Ok(out)
    }
}

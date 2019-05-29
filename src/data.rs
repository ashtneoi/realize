use ron;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

type Id = [u8; 64];

/// A thing that transforms artifacts. Specified with a command list.
struct Action {
    cmds: Vec<String>,
}

impl Action {
    fn apply(&self) -> ReverseAction {
        unimplemented!();
    }
}

#[derive(Deserialize, Serialize)]
struct ReverseAction {
    id: Id,
    subactions: Vec<Subaction>,
}

/// A cacheable part of an Action. Involves a single file.
#[derive(Deserialize, Serialize)]
enum Subaction {
    Add {
        path: PathBuf,
        data: Id,
    },
    Remove {
        path: PathBuf,
    },
}

impl Subaction {
    fn apply(self) {
        use Subaction::*;
        match self {
            Add { path, data } => panic!(),
            Remove { path } => panic!(),
        }
    }
}

struct Artifact {
    id: Id,
}

struct Cache {
    path: PathBuf,
}

impl Cache {
    fn new() -> Self {
        Cache { path: PathBuf::from(".realize/cache") }
    }

    fn get(&self, id: Id) -> ReverseAction {
    }

    fn put(&self, action: &ReverseAction) {
        // TODO: Can we do this more efficiently?
        let mut action_file = PathBuf::from(".realize");
        base.push(&action.id[0..2]);
        base.push(&action.id[2..action.id.len()]);
        fs::create_dir_all(base.parent());
        let r = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&base);
        match r {
            Err(e) => {
                if e.kind() == ErrorKind::AlreadyExists {
                    panic!(); // FIXME: or something
                } else {
                    return Err(e);
                }
            },
            Ok(f) => {
                f.write("something");
            },
        }
    }
}

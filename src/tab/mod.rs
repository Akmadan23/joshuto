mod homepage;
mod new_tab_mode;
mod options;

pub use homepage::*;
pub use new_tab_mode::*;
pub use options::*;

use std::collections::HashMap;
use std::path;

use crate::fs::JoshutoDirList;
use crate::history::JoshutoHistory;
use crate::preview::preview_dir::PreviewDirState;

type HistoryMetadata = HashMap<path::PathBuf, PreviewDirState>;

pub struct JoshutoTab {
    pub cwd: path::PathBuf,
    // history is just a HashMap, so we have this property to store last workdir
    pub previous_dir: Option<path::PathBuf>,
    pub history: JoshutoHistory,
    pub history_metadata: HistoryMetadata,
    pub options: TabDisplayOption,
}

impl JoshutoTab {
    pub fn new(
        cwd: path::PathBuf,
        history: JoshutoHistory,
        tab_options: TabDisplayOption,
    ) -> std::io::Result<Self> {
        let new_tab = Self {
            cwd,
            previous_dir: None,
            history,
            history_metadata: HashMap::new(),
            options: tab_options,
        };

        Ok(new_tab)
    }

    pub fn option_ref(&self) -> &TabDisplayOption {
        &self.options
    }

    pub fn option_mut(&mut self) -> &mut TabDisplayOption {
        &mut self.options
    }

    pub fn get_cwd(&self) -> &path::Path {
        self.cwd.as_path()
    }
    pub fn set_cwd(&mut self, cwd: &path::Path) {
        self.previous_dir = Some(self.cwd.to_path_buf());
        self.cwd = cwd.to_path_buf();

        // OSC 7: Escape sequence to set the working directory
        // print!("\x1b]7;file://{}{}\x1b\\", HOSTNAME.as_str(), cwd.display());
    }

    pub fn previous_dir(&self) -> Option<&path::Path> {
        self.previous_dir.as_deref()
    }

    pub fn history_ref(&self) -> &JoshutoHistory {
        &self.history
    }
    pub fn history_mut(&mut self) -> &mut JoshutoHistory {
        &mut self.history
    }

    pub fn history_metadata_ref(&self) -> &HistoryMetadata {
        &self.history_metadata
    }
    pub fn history_metadata_mut(&mut self) -> &mut HistoryMetadata {
        &mut self.history_metadata
    }

    pub fn curr_list_ref(&self) -> Option<&JoshutoDirList> {
        self.history.get(self.get_cwd())
    }
    pub fn parent_list_ref(&self) -> Option<&JoshutoDirList> {
        let parent = self.get_cwd().parent()?;
        self.history.get(parent)
    }
    pub fn child_list_ref(&self) -> Option<&JoshutoDirList> {
        let curr_list = self.curr_list_ref()?;
        let index = curr_list.get_index()?;
        let path = curr_list.contents[index].file_path();
        self.history.get(path)
    }

    pub fn curr_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        self.history.get_mut(self.cwd.as_path())
    }
    pub fn parent_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        let parent = self.cwd.parent()?;
        self.history.get_mut(parent)
    }
    #[allow(dead_code)]
    pub fn child_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        let child_path = {
            let curr_list = self.curr_list_ref()?;
            let index = curr_list.get_index()?;
            curr_list.contents[index].file_path().to_path_buf()
        };

        self.history.get_mut(child_path.as_path())
    }
}

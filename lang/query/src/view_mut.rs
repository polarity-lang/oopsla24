use codespan::FileId;

use crate::*;

/// Mutable view on a file in the database
pub struct DatabaseViewMut<'a> {
    pub(crate) file_id: FileId,
    pub(crate) database: &'a mut Database,
}

impl<'a> DatabaseViewMut<'a> {
    pub fn load(&mut self) -> Result<(), Error> {
        self.database.index.modify(self.file_id, |mut index| index.reset());
        let prg = self.query_ref().tst()?;
        self.database.index.modify(self.file_id, |mut index| {
            let (info_lapper, item_lapper) = collect_info(&prg);
            index.set(info_lapper, item_lapper);
        });
        Ok(())
    }

    pub fn update(&mut self, source: String) -> Result<(), Error> {
        let DatabaseViewMut { file_id, database } = self;
        database.files.update(*file_id, source);
        self.load()
    }

    pub fn query(self) -> DatabaseView<'a> {
        let DatabaseViewMut { file_id, database } = self;
        DatabaseView { file_id, database }
    }

    pub fn query_ref(&self) -> DatabaseView<'_> {
        let DatabaseViewMut { file_id, database } = self;
        DatabaseView { file_id: *file_id, database }
    }
}

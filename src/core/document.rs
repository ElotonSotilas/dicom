use std::{
    fs::{self, File},
    io::{Read, Seek, Write},
    path::PathBuf,
    time::SystemTime,
};

use super::{
    dataset::Dataset,
    error::{DicomError, DicomResult},
    tag::{DicomTag, DicomValue},
};

pub trait Document {
    fn open(path: &str) -> DicomResult<Self>
    where
        Self: Sized;
    fn refresh(&mut self) -> ();
    fn read(&mut self) -> DicomResult<&Dataset>;
    fn write(&mut self, dataset: &Dataset) -> DicomResult<()>;
    fn close(&mut self) -> DicomResult<()>;
    fn is_open(&self) -> bool;
    fn is_modified(&self) -> bool;
    fn get_path(&self) -> Option<&str>;
    fn get_name(&self) -> Option<&str>;
    fn get_extension(&self) -> Option<&str>;
    fn get_size(&self) -> Option<usize>;
    fn get_creation_date(&self) -> Option<SystemTime>;
    fn get_modification_date(&self) -> Option<SystemTime>;
    fn get_access_date(&self) -> Option<SystemTime>;
}

#[derive(PartialEq)]
pub enum DocumentState {
    Open,
    Closed,
    Modified,
}

#[derive(PartialEq)]
pub enum DocumentMode {
    ReadOnly,
    ReadWrite,
}

#[derive(PartialEq)]
pub enum WritingMode {
    Append,
    Overwrite,
    Truncate,
}

pub struct DicomDocument {
    file: File,
    path: Option<PathBuf>,
    state: DocumentState,
    dataset: Option<Dataset>,
    mode: DocumentMode,
    writer: WritingMode,
    should_sync: bool,
}

impl Document for DicomDocument {
    fn open(path: &str) -> DicomResult<Self> {
        let mut _this = None;
        if PathBuf::from(path).exists() {
            let file = File::options().append(true).open(path)?;
            let writer = WritingMode::Append;
            let state = DocumentState::Open;
            _this = Some(DicomDocument {
                file,
                state,
                dataset: None,
                mode: DocumentMode::ReadWrite,
                writer,
                path: Some(PathBuf::from(path)),
                should_sync: true,
            });
        } else {
            let file = File::create(path)?;
            let writer = WritingMode::Overwrite;
            let state = DocumentState::Open;
            _this = Some(DicomDocument {
                file,
                state,
                dataset: None,
                mode: DocumentMode::ReadWrite,
                writer,
                path: Some(PathBuf::from(path)),
                should_sync: true,
            });
        }

        Ok(_this.unwrap())
    }

    fn refresh(&mut self) {
        self.should_sync = true;
    }

    fn read(&mut self) -> DicomResult<&Dataset> {
        if self.should_sync {
            let mut buffer = String::new();

            self.file.read_to_string(&mut buffer)?;
            let dataset = parse_dicom(&buffer)?;

            self.dataset = Some(dataset);
            self.state = DocumentState::Closed;

            self.should_sync = false;
        }

        Ok(self.dataset.as_ref().unwrap())
    }

    fn write(&mut self, dataset: &Dataset) -> DicomResult<()> {
        if self.mode == DocumentMode::ReadOnly {
            return Err(DicomError::IOError("Document is read-only".to_string()));
        }

        if self.writer == WritingMode::Truncate {
            self.file.set_len(0)?;
        }

        else if self.writer == WritingMode::Append {
            self.file.seek(std::io::SeekFrom::End(0))?;
        }

        self.file.write_all(dataset.to_string().as_bytes())?;
        self.state = DocumentState::Modified;

        Ok(())
    }

    fn close(&mut self) -> DicomResult<()> {
        self.file.sync_all()?;

        self.path = None;
        self.state = DocumentState::Closed;
        self.mode = DocumentMode::ReadOnly;

        Ok(())
    }

    fn is_open(&self) -> bool {
        match self.state {
            DocumentState::Open => true,
            _ => false,
        }
    }

    fn is_modified(&self) -> bool {
        match self.state {
            DocumentState::Modified => true,
            _ => false,
        }
    }

    fn get_path(&self) -> Option<&str> {
        self.path.as_ref().map(|p| p.to_str().unwrap())
    }

    fn get_name(&self) -> Option<&str> {
        self.path
            .as_ref()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
    }

    fn get_extension(&self) -> Option<&str> {
        self.path
            .as_ref()
            .map(|p| p.extension().unwrap().to_str().unwrap())
    }

    fn get_size(&self) -> Option<usize> {
        self.path
            .as_ref()
            .map(|p| fs::metadata(p).unwrap().len() as usize)
    }

    fn get_creation_date(&self) -> Option<SystemTime> {
        self.path
            .as_ref()
            .map(|p| fs::metadata(p).unwrap().created().unwrap())
    }

    fn get_modification_date(&self) -> Option<SystemTime> {
        self.path
            .as_ref()
            .map(|p| fs::metadata(p).unwrap().modified().unwrap())
    }

    fn get_access_date(&self) -> Option<SystemTime> {
        self.path
            .as_ref()
            .map(|p| fs::metadata(p).unwrap().accessed().unwrap())
    }
}

fn parse_dicom(input: &str) -> DicomResult<Dataset> {
    unimplemented!()
}

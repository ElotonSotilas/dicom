use std::{collections::VecDeque, fmt::Display, rc::Rc};

use super::tag::DicomTag;

pub type CursorPosition = usize;

#[derive(Debug, Clone)]
pub struct Dataset {
    objects: VecDeque<Rc<dyn DicomTag>>,
}

impl Dataset {
    pub fn new() -> Self {
        Dataset {
            objects: VecDeque::new(),
        }
    }

    pub fn get(&self, position: CursorPosition) -> Option<&Rc<dyn DicomTag>> {
        self.objects.get(position)
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn push_back(&mut self, dicom_object: Rc<dyn DicomTag>) {
        self.objects.push_back(dicom_object);
    }

    pub fn push_front(&mut self, dicom_object: Rc<dyn DicomTag>) {
        self.objects.push_front(dicom_object);
    }

    pub fn insert(&mut self, position: CursorPosition, dicom_object: Rc<dyn DicomTag>) {
        if position < self.objects.len() {
            self.objects.insert(position, dicom_object);
        } else {
            panic!("Position {} out of bounds", position);
        }
    }

    pub fn remove_at(&mut self, position: CursorPosition) -> Option<Rc<dyn DicomTag>> {
        if position < self.objects.len() {
            self.objects.remove(position)
        } else {
            None
        }
    }

    pub fn replace_at(&mut self, position: CursorPosition, dicom_object: Rc<dyn DicomTag>) -> Option<&Rc<dyn DicomTag>> {
        if position < self.objects.len() {
            self.objects.insert(position, dicom_object);
            self.objects.get(position)
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<Rc<dyn DicomTag>> {
        self.objects.pop_back()
    }

    pub fn pop_front(&mut self) -> Option<Rc<dyn DicomTag>> {
        self.objects.pop_front()
    }
}

// Implementing Iterator for Dataset
impl<'a> IntoIterator for &'a Dataset {
    type Item = &'a Rc<dyn DicomTag>;
    type IntoIter = std::collections::vec_deque::Iter<'a, Rc<dyn DicomTag>>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.iter()
    }
}

impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for object in &self.objects {
            write!(f, "{}", object)?;
        }

        Ok(())
    }
}
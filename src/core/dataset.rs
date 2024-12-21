use std::collections::VecDeque;

use super::tag::DicomTag;

type CursorPosition = usize;

pub struct Dataset {
    objects: VecDeque<Box<dyn DicomTag>>,
}

impl Dataset {
    pub fn new() -> Self {
        Dataset {
            objects: VecDeque::new(),
        }
    }

    pub fn get(&self, position: CursorPosition) -> Option<&Box<dyn DicomTag>> {
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

    pub fn push_back(&mut self, dicom_object: Box<dyn DicomTag>) {
        self.objects.push_back(dicom_object);
    }

    pub fn push_front(&mut self, dicom_object: Box<dyn DicomTag>) {
        self.objects.push_front(dicom_object);
    }

    pub fn insert(&mut self, position: CursorPosition, dicom_object: Box<dyn DicomTag>) {
        if position < self.objects.len() {
            self.objects.insert(position, dicom_object);
        } else {
            panic!("Position {} out of bounds", position);
        }
    }

    pub fn remove_at(&mut self, position: CursorPosition) -> Option<Box<dyn DicomTag>> {
        if position < self.objects.len() {
            self.objects.remove(position)
        } else {
            None
        }
    }

    pub fn replace_at(&mut self, position: CursorPosition, dicom_object: Box<dyn DicomTag>) -> Option<&Box<dyn DicomTag>> {
        if position < self.objects.len() {
            self.objects.insert(position, dicom_object);
            self.objects.get(position)
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<Box<dyn DicomTag>> {
        self.objects.pop_back()
    }

    pub fn pop_front(&mut self) -> Option<Box<dyn DicomTag>> {
        self.objects.pop_front()
    }
}

// Implementing Iterator for Dataset
impl<'a> IntoIterator for &'a Dataset {
    type Item = &'a Box<dyn DicomTag>;
    type IntoIter = std::collections::vec_deque::Iter<'a, Box<dyn DicomTag>>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.iter()
    }
}

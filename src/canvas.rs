// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{color::Color, document::Document};

/// # Class Invariant
///   - m_history.len() >= 1
#[derive(Clone, Debug)]
pub struct DrawingCanvas {
    /// History of document states.
    m_history: Vec<Document>,

    /// Currently selected document index.
    m_selected: usize,

    /// Visible document to render.
    m_visible_doc: Document,
}

impl DrawingCanvas {
    /// Creates a new drawing canvas.
    pub fn new(width: usize, height: usize, bg_color: Color) -> Self {
        let doc = Document::new(width, height, bg_color);
        DrawingCanvas {
            m_history: vec![doc.clone()],
            m_selected: 0,
            m_visible_doc: doc,
        }
    }

    /// Removes all the document after currently selected document in history
    /// and the add visible document to history.
    pub fn commit(&mut self) {
        self.m_history.truncate(self.m_selected + 1);
        self.m_history.push(self.m_visible_doc.clone());
    }

    /// Undo the last operation and changes visible document to currently selected state.
    /// Returns true if there is something to undo otherwise returns false.
    pub fn undo(&mut self) -> bool {
        if self.m_selected == 0 {
            return false;
        }
        self.m_selected -= 1;
        self.m_visible_doc = self.m_history[self.m_selected].clone();
        true
    }

    /// Redo the last undo operation and changes visible document to currently selected state.
    /// Returns true if there is something to redo otherwise returns false.
    pub fn redo(&mut self) -> bool {
        if self.m_selected == self.m_history.len() - 1 {
            return false;
        }
        self.m_selected += 1;
        self.m_visible_doc = self.m_history[self.m_selected].clone();
        true
    }

    /// Returns reference to visible document.
    pub fn visible_doc(&self) -> &Document {
        &self.m_visible_doc
    }

    /// Returns mutable reference to visible document.
    pub fn visible_doc_mut(&mut self) -> &mut Document {
        &mut self.m_visible_doc
    }

    /// Returns reference to history of document.
    pub fn history(&self) -> &Vec<Document> {
        &self.m_history
    }

    /// Returns index of current state of document in history.
    pub fn current_idx(&self) -> usize {
        self.m_selected
    }

    /// Returns index of current state of document in history.
    pub fn current(&self) -> &Document {
        &self.m_history[self.m_selected]
    }

    /// Clear history from given index.
    ///
    /// # Precondition
    ///   - i < self.hisotry().len()
    ///   - i > 0
    pub fn clear_history_from(&mut self, i: usize) {
        self.m_history.truncate(i + 1);
        self.m_selected = i;
    }

    /// Erases history element at given index.
    ///
    /// # Precondition
    ///   - i < self.history().len()
    ///   - self.history().len() > 1
    pub fn erase_history_element(&mut self, i: usize) {
        self.m_history.remove(i);
        if i <= self.m_selected {
            self.m_selected -= 1;
        }
    }
}

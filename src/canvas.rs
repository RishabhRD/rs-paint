// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{color::Color, document::Document};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DrawingCanvas {
    /// Document that decides what would be saved in disk after on save.
    m_final_doc: Document,
    /// Document that is visible on screen.
    m_visible_doc: Document,
    /// Undo stack of current canvas.
    m_undo_stack: Vec<Document>,
    /// Redo stack of current canvas.
    m_redo_stack: Vec<Document>,
}

impl DrawingCanvas {
    /// Creates a new drawing canvas.
    pub fn new(width: usize, height: usize, bg_color: Color) -> Self {
        let doc = Document::new(width, height, bg_color);
        DrawingCanvas {
            m_final_doc: doc.clone(),
            m_visible_doc: doc,
            m_undo_stack: vec![],
            m_redo_stack: vec![],
        }
    }

    /// Commits visible document as final document.
    ///
    /// # Postcondition
    ///   - Makes visible document as final document that can be saved in disk.
    ///   - Clears the redo stack.
    pub fn commit_visible(&mut self) {
        self.m_final_doc = self.m_visible_doc.clone();
        self.m_redo_stack.clear();
    }

    /// Replaces final and visible document with last undo state.
    ///
    /// # Postcondition
    ///   - If m_undo_stack is empty, do nothing.
    ///   - m_final_doc is replaced with last undo state.
    ///   - m_visible_doc is replaced with last undo state.
    ///   - add last undo state to redo state.
    pub fn undo(&mut self) {
        match self.m_undo_stack.pop() {
            None => (),
            Some(undo_doc) => {
                self.m_final_doc = undo_doc.clone();
                self.m_visible_doc = undo_doc.clone();
                self.m_redo_stack.push(undo_doc.clone());
            }
        }
    }

    /// Replaces final and visible document with last redo state.
    ///
    /// # Postcondition
    ///   - If m_redo_stack is empty, do nothing.
    ///   - m_final_doc is replaced with last redo state.
    ///   - m_visible_doc is replaced with last redo state.
    ///   - add last redo state to undo state.
    pub fn redo(&mut self) {
        match self.m_redo_stack.pop() {
            None => (),
            Some(redo_doc) => {
                self.m_final_doc = redo_doc.clone();
                self.m_visible_doc = redo_doc.clone();
                self.m_undo_stack.push(redo_doc.clone());
            }
        }
    }

    /// Returns reference to visible document.
    pub fn visible_doc(&self) -> &Document {
        &self.m_visible_doc
    }

    /// Returns mutable reference to visible document.
    pub fn visible_doc_mut(&mut self) -> &mut Document {
        &mut self.m_visible_doc
    }

    /// Returns reference to final document.
    pub fn final_doc(&self) -> &Document {
        &self.m_final_doc
    }
}

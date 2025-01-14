use std::fmt;

use crate::dic::grammar::Grammar;
use crate::dic::lexicon::word_infos::WordInfo;
use crate::dic::lexicon_set::LexiconSet;
use crate::prelude::*;

/// Lattice node
#[derive(Clone, Debug, Default)]
pub struct Node {
    /// The byte_idx begin of node
    pub begin: usize,
    /// The byte_idx end of node
    pub end: usize,

    /// The left_id to calculate connection cost
    pub left_id: i16,
    /// The right_id to calculate connection cost
    pub right_id: i16,
    /// The cost of this node
    pub cost: i16,

    /// The word_id in the dictionary of this node
    /// None if this node comes from outside of dictionary (e.g. oov)
    pub word_id: Option<u32>,
    /// The word_info in the dictionary of this node
    /// None until the first time it is referenced
    pub word_info: Option<WordInfo>,
    /// Wherther if this node is oov
    pub is_oov: bool,

    /// The total cost from bos to this node
    pub total_cost: i32,
    /// The node idx in the lattice of the best previous node
    pub best_previous_node_index: Option<(usize, usize)>,
    /// Whether if this node is connecting to bos node
    pub is_connected_to_bos: bool,
}

impl Node {
    /// Creates a node with word params and word_id
    pub fn new(left_id: i16, right_id: i16, cost: i16, word_id: u32) -> Node {
        Node {
            left_id,
            right_id,
            cost,
            word_id: Some(word_id),
            ..Default::default()
        }
    }

    /// Sets begin and end
    pub fn set_range(&mut self, begin: usize, end: usize) {
        self.begin = begin;
        self.end = end;
    }

    /// Sets word_info
    pub fn set_word_info(&mut self, word_info: WordInfo) {
        self.word_info = Some(word_info);
    }

    /// Consult dictionary and sets word_info
    pub fn fill_word_info(&mut self, lexicon: &LexiconSet) -> SudachiResult<()> {
        if let None = &self.word_info {
            let word_id = self.word_id.ok_or(SudachiError::MissingWordId)?;
            self.set_word_info(lexicon.get_word_info(word_id)?);
        }
        Ok(())
    }

    /// Returns if the node has word_info (possibly in the dictionary)
    pub fn is_defined(&self) -> bool {
        match (&self.word_id, &self.word_info) {
            (None, None) => false,
            _ => true,
        }
    }

    /// Return dictionary id where the word is defined
    ///
    /// Return -1 if the word is not in any of dictionaries
    pub fn get_dictionary_id(&self) -> i32 {
        if let Some(wi) = &self.word_id {
            return LexiconSet::get_dictionary_id(*wi) as i32;
        }
        -1
    }

    /// Create a BOS node
    pub fn new_bos() -> Node {
        let (left_id, right_id, cost) = Grammar::BOS_PARAMETER;
        Node {
            left_id,
            right_id,
            cost,
            is_connected_to_bos: true,
            ..Default::default()
        }
    }

    /// Create a EOS node
    pub fn new_eos(size: usize) -> Node {
        let (left_id, right_id, cost) = Grammar::EOS_PARAMETER;
        Node {
            begin: size,
            end: size,
            left_id,
            right_id,
            cost,
            ..Default::default()
        }
    }

    /// Create a out_of_vocabulary node
    pub fn new_oov(left_id: i16, right_id: i16, cost: i16, word_info: WordInfo) -> Node {
        Node {
            left_id,
            right_id,
            cost,
            word_id: None,
            word_info: Some(word_info),
            is_oov: true,
            ..Default::default()
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}({}) {} {} {}",
            self.begin,
            self.end,
            match &self.word_info {
                Some(wi) => wi.surface.clone(),
                None => "".to_string(),
            },
            match self.word_id {
                Some(word_id) => word_id.to_string(),
                None => "-1".to_string(),
            },
            self.left_id,
            self.right_id,
            self.cost
        )
    }
}

use std::cell::RefCell;

use swc_common::comments::{Comment, Comments};
use swc_common::BytePos;

#[derive(Default)]
pub struct SequentialComments {
  annotations: RefCell<Vec<AnnotationWithType>>,
}

impl SequentialComments {
  pub fn add_comment(&self, comment: Comment) {
    if comment.text.starts_with('#') && comment.text.contains("sourceMappingURL=") {
      self.annotations.borrow_mut().push(AnnotationWithType {
        comment,
        kind: AnnotationKind::SourceMappingUrl,
      });
      return;
    }
    let mut search_position = 1;
    while let Some(Some(match_position)) = comment.text.get(search_position..).map(|s| s.find("__"))
    {
      search_position += match_position;
      match &comment.text[search_position - 1..search_position] {
        "@" | "#" => {
          let annotation_slice = &comment.text[search_position..];
          if annotation_slice.starts_with("__PURE__") {
            self.annotations.borrow_mut().push(AnnotationWithType {
              comment,
              kind: AnnotationKind::Pure,
            });
            return;
          }
          if annotation_slice.starts_with("__NO_SIDE_EFFECTS__") {
            self.annotations.borrow_mut().push(AnnotationWithType {
              comment,
              kind: AnnotationKind::NoSideEffects,
            });
            return;
          }
        }
        _ => {}
      }
      search_position += 3;
    }
  }

  pub fn take_annotations(self) -> Vec<AnnotationWithType> {
    self.annotations.take()
  }
}

impl Comments for SequentialComments {
  fn add_leading(&self, _: BytePos, comment: Comment) {
    self.add_comment(comment);
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn add_leading_comments(&self, _: BytePos, _: Vec<Comment>) {
    panic!("add_leading_comments");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn has_leading(&self, _: BytePos) -> bool {
    panic!("has_leading");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn move_leading(&self, _: BytePos, _: BytePos) {
    panic!("move_leading");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn take_leading(&self, _: BytePos) -> Option<Vec<Comment>> {
    panic!("take_leading");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn get_leading(&self, _: BytePos) -> Option<Vec<Comment>> {
    panic!("get_leading");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn add_trailing(&self, _: BytePos, comment: Comment) {
    self.add_comment(comment);
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn add_trailing_comments(&self, _: BytePos, _: Vec<Comment>) {
    panic!("add_trailing_comments");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn has_trailing(&self, _: BytePos) -> bool {
    panic!("has_trailing");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn move_trailing(&self, _: BytePos, _: BytePos) {
    panic!("move_trailing");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn take_trailing(&self, _: BytePos) -> Option<Vec<Comment>> {
    panic!("take_trailing");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn get_trailing(&self, _: BytePos) -> Option<Vec<Comment>> {
    panic!("get_trailing");
  }

  #[cfg_attr(not(debug_assertions), inline(always))]
  fn add_pure_comment(&self, _: BytePos) {
    panic!("add_pure_comment");
  }
}

pub struct AnnotationWithType {
  pub comment: Comment,
  pub kind: AnnotationKind,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AnnotationKind {
  Pure,
  NoSideEffects,
  SourceMappingUrl,
}

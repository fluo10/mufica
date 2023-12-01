use std::collections::HashMap;
use std::sync::Arc;
use std::ops::{DerefMut, Deref};
use imbl::Vector;
use matrix_sdk_ui::timeline::TimelineItem;

use crate::Result;

use matrix_sdk::{
    ruma::{
        OwnedRoomId,
        UInt,
    },
    room::{Messages, MessagesOptions},
    deserialized_responses::TimelineEvent,
    Client,
};

#[derive(Clone, Debug)]
pub struct MatrixTimeline{
    pub inner: Vector<Arc<TimelineItem>>,
}

impl MatrixTimeline {
    pub fn new () -> Self {
        Self{
            inner: Vector::new()
        }
    }
    pub fn append(&mut self, v: Vector<Arc<TimelineItem>>) {
        self.inner.append(v);
    }
    pub fn push_back(&mut self, t: Arc<TimelineItem>) {
        self.inner.push_back(t);
    }
}

impl Deref for MatrixTimeline {
    type Target = Vector<Arc<TimelineItem>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for MatrixTimeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}



impl From<Vector<Arc<TimelineItem>>> for MatrixTimeline {
    fn from(v: Vector<Arc<TimelineItem>>) -> Self {
        Self{
            inner: v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_history_to_plain() {
        todo!()
    }
}

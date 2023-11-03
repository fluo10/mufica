use std::collections::HashMap;

use matrix_sdk::{
    ruma::{
        OwnedRoomId,
        UInt,
    },
    room::{Messages, MessagesOptions},
    deserialized_responses::TimelineEvent,
};

#[derive(Clone, Debug)]
pub struct MatrixHistory{
    pub inner: HashMap<OwnedRoomId, Vec<TimelineEvent>>
}


impl MatrixHistory {
}

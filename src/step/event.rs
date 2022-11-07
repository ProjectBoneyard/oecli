//! During the lifecycle of a step, several events take place. At the start of a [StepItem]() being
//! processed is considered an event, as well as end, starting a new sequence, or ending one. As
//! well as skipped steps.

/// During the lifecycle of a step, the following events can be emitted:
pub enum StepEvent<'a> {
    End(&'a EventData),
    Error(&'a EventData),
    Start(&'a EventData),
    NewSequence(NewSequenceEvent),
    EndSequence(&'a EventData),
    Skip(&'a EventData),
}

/// When the Executor publishes an event, that event can provide additional information and
/// context.
pub struct EventData {
    pub msg: String,
    pub sequence_name: String,
}

impl EventData {
    pub fn new(msg: &str, sequence_name: &str) -> EventData {
        EventData {
            msg: msg.to_string(),
            sequence_name: sequence_name.to_string(),
        }
    }
}

/// When the Executor beings processing a new StepSequence it publishes information about the
/// sequence that the end user may want to know about.
pub struct NewSequenceEvent {
    pub length: usize,
    pub sequence_name: String,
}

impl NewSequenceEvent {
    pub fn new(length: usize, sequence_name: &str) -> NewSequenceEvent {
        NewSequenceEvent {
            length,
            sequence_name: sequence_name.to_string(),
        }
    }
}

//! This module defines the behaviour of protocol parties in the different
//! phases of the protocol.

use hacspec_lib::Randomness;

use crate::{
    circuit::Circuit,
    messages::{Message, MessagePayload, SubMessage},
    primitives::mac::MacKey,
    Error,
};
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
/// A type for tracking the current protocol phase.
pub enum ProtocolPhase {
    /// Before the protocol has begun
    PreInit,
    /// Function-independent pre-processing
    Init,
}

/// Collects all party communication channels.
///
/// It includes
/// - `listen`: The parties on message receiver handle
/// - `evaluator`: The sender handle for the designated evaluator party
/// - `parties`: All other parties' sender handles, ordered by their `id`s
/// - `id`: The owning parties `id`
#[allow(dead_code)] // TODO: Remove this later.
pub struct ChannelConfig {
    pub(crate) listen: Receiver<Message>,
    pub(crate) evaluator: Sender<Message>,
    pub(crate) parties: Vec<Sender<Message>>,
    pub(crate) id: usize,
}

/// A struct defining protocol party state during a protocol execution.
#[allow(dead_code)] // TODO: Remove this later.
pub struct Party {
    /// The parties numeric identifier
    id: usize,
    /// The channel configuration for communicating to other protocol parties
    channels: ChannelConfig,
    /// The global MAC key for authenticating bit shares
    global_mac_key: Option<MacKey>,
    /// The circuit to be evaluated during the multi-party computation
    circuit: Circuit,
    /// A source of random bytes and bits local to the party
    entropy: Randomness,
    /// Tracks the current phase of protocol execution
    current_phase: ProtocolPhase,
}

#[allow(dead_code)] // TODO: Remove this later.
impl Party {
    /// Initialize an MPC party.
    pub fn new(channels: ChannelConfig, circuit: &Circuit, entropy: Randomness) -> Self {
        Self {
            id: channels.id,
            channels,
            global_mac_key: None,
            circuit: circuit.clone(),
            entropy,
            current_phase: ProtocolPhase::PreInit,
        }
    }

    /// Return `true`, if the party is the designated circuit evaluator.
    fn is_evaluator(&self) -> bool {
        self.id == 0
    }

    /// Send a message to the evaluator.
    fn send_to_evaluator(&mut self, message: Message) {
        self.channels.evaluator.send(message).unwrap();
    }

    fn ot_round(&mut self) -> Result<Vec<Vec<u8>>, Error> {
        let num_parties = self.channels.parties.len();

        let mut ot_results = Vec::new();
        // Expect earlier parties' messages.
        for _i in 0..self.id {
            let choose_left = self.entropy.bit()?;
            ot_results.push(self.ot_receive(choose_left)?);
        }

        // All earlier messages have been received, so it is the parties' turn
        // to send messages to everyone, except itself.
        for i in 0..num_parties {
            if i == self.id {
                continue;
            }
            let left_input = [1u8, 1u8, 1u8];
            let right_input = [9u8, 9u8, 9u8];
            self.ot_send(i, &left_input, &right_input)?;
        }

        // Wait for the messages sent by later parties.
        for _i in self.id + 1..num_parties {
            let choose_left = self.entropy.bit()?;
            ot_results.push(self.ot_receive(choose_left)?);
        }

        Ok(ot_results)
    }

    fn ot_send(&mut self, i: usize, left_input: &[u8], right_input: &[u8]) -> Result<(), Error> {
        let (own_sender, own_receiver) = mpsc::channel::<SubMessage>();
        let (their_sender, their_receiver) = mpsc::channel::<SubMessage>();

        let channel_msg = Message {
            from: self.id,
            to: i,
            payload: MessagePayload::SubChannel(own_sender, their_receiver),
        };
        self.channels.parties[i].send(channel_msg).unwrap();

        let dst = format!("OT-{}-{}", self.id, i);
        let (ot_sender, ot_commit) =
            crate::primitives::ot::OTSender::init(&mut self.entropy, dst.as_bytes())?;
        their_sender.send(SubMessage::OTCommit(ot_commit)).unwrap();
        let receiver_msg = own_receiver.recv().unwrap();
        match receiver_msg {
            SubMessage::OTSelect(selection) => {
                let send =
                    ot_sender.send(left_input, right_input, &selection, &mut self.entropy)?;
                their_sender.send(SubMessage::OTSend(send)).unwrap();
            }
            _ => panic!(
                "Party {} - Unexpected receiver message in OT:\n{receiver_msg:#?}",
                self.id
            ),
        };
        Ok(())
    }

    fn ot_receive(&mut self, choose_left: bool) -> Result<Vec<u8>, Error> {
        let channel_msg = self.channels.listen.recv().unwrap();

        Ok(match channel_msg {
            Message {
                to,
                from,
                payload: MessagePayload::SubChannel(their_channel, my_channel),
            } => {
                let first_msg = my_channel.recv().unwrap();
                match first_msg {
                    SubMessage::OTCommit(commitment) => {
                        let dst = format!("OT-{}-{}", from, to);
                        self.log(&format!("Choose left input: {choose_left}"));
                        let (receiver, resp) = crate::primitives::ot::OTReceiver::select(
                            &mut self.entropy,
                            dst.as_bytes(),
                            commitment,
                            choose_left,
                        )?;
                        their_channel.send(SubMessage::OTSelect(resp)).unwrap();
                        let second_msg = my_channel.recv().unwrap();
                        match second_msg {
                            SubMessage::OTSend(payload) => {
                                assert_eq!(self.id, to);
                                let result = receiver.receive(payload)?;
                                self.log(&format!("Got message {result:?}"));
                                result
                            }
                            _ => panic!("Unexpected second sender message in OT"),
                        }
                    }
                    _ => panic!("Unexpected first sender message in OT"),
                }
            }
            _ => panic!("Unexpected channel message in OT"),
        })
    }

    /// Run the function independent pre-processing phase of the protocol.
    pub fn function_independent(&mut self) {
        todo!("the function-independent pre-processing phase is not yet implemented (cf. GitHub issue #51")
    }

    /// Run the function-dependent pre-processing phase of the protocol.
    pub fn function_dependent(&mut self) {
        todo!("the function-dependent pre-processing phase is not yet implemented (cf. GitHub issue #51")
    }

    /// Run the input-processing phase of the protocol.
    pub fn input_processing(&mut self) {
        todo!("the input processing phase is not yet implemented (cf. GitHub issue #52")
    }

    /// Run the circuit evaluation phase of the protocol.
    pub fn evaluate_circuit(&mut self) {
        todo!("the circuit evaluation phase is not yet implemented (cf. GitHub issue #54")
    }

    /// Run the output processing phase of the protocol
    pub fn output_processing(&mut self) {
        todo!("the output processing phase is not yet implemented (cf. GitHub issue #53")
    }

    /// Run the MPC protocol, returning the parties output, if any.
    pub fn run(&mut self) -> Result<Option<Vec<bool>>, Error> {
        self.log("Running OTs with every other party.");

        self.ot_round()?;
        Ok(None)
    }

    /// Utility function to provide debug output during the protocol run.
    fn log(&self, message: &str) {
        eprintln!(
            "Party {} in phase {:?}: {}",
            self.id, self.current_phase, message
        );
    }
}

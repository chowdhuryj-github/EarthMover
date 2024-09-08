use std::marker::PhantomData;

use crate::goals::{Goal, Rewardable};

use super::buffer::BufferMarker;

pub struct Untrained;
pub struct InReview;

/// The agent will hold both the current goal of the system and a reference to the actual hardware
/// we're using. After sufficient data from the environment has been collected, the goal, agent
/// body, and data buf will all be sent to the remote simulation engine to before parralellized RL
/// The response will then be a finished AgentSession that can be attempted to run in the field!
pub struct AgentSession<'agent, REWARD: Rewardable, STATE, const BUFFER_SIZE: usize> {
    goal: Goal<REWARD>,
    body: &'agent Body,
    data_buf: [u8; BUFFER_SIZE],
    buffer_pos: BufferMarker<BUFFER_SIZE>,

    directions: Option<Vec<u8>>,
    _spooky_ghost: PhantomData<STATE>,
}

impl<'agent, REWARD: Rewardable, STATE, const BUFFER_SIZE: usize>
    AgentSession<'agent, REWARD, STATE, BUFFER_SIZE>
{
    pub fn builder() -> Builder<'agent, REWARD, BUFFER_SIZE> {
        Builder::default()
    }
}

impl<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize>
    AgentSession<'agent, REWARD, InReview, BUFFER_SIZE>
{
    pub fn get_directions(&self) -> Option<&[u8]> {
        self.directions.as_deref()
    }
}

/// Builder will create a new agent session from a Body's reference. The STATE of this
/// AgentSession will always be untrained. Only when receiving a new Agent back from the simulation
/// server will we receive an agent tagged as Trained. Untrained agents do not have access to the
/// directions bytes, preventing them from being runnable
pub struct Builder<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize> {
    goal: Option<Goal<REWARD>>,
    body: Option<&'agent Body>,
}

impl<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize> Default
    for Builder<'agent, REWARD, BUFFER_SIZE>
{
    fn default() -> Self {
        Self {
            goal: None,
            body: None,
        }
    }
}

impl<'agent, REWARD: Rewardable, const BUFFER_SIZE: usize> Builder<'agent, REWARD, BUFFER_SIZE> {
    pub fn with_goal(mut self, goal: Goal<REWARD>) -> Self {
        self.goal = Some(goal);
        self
    }

    pub fn with_body(mut self, body: &'agent Body) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Option<AgentSession<'agent, REWARD, Untrained, BUFFER_SIZE>> {
        match (self.goal, self.body) {
            (Some(goal), Some(body)) => Some(AgentSession {
                goal,
                body,
                data_buf: [0u8; BUFFER_SIZE],
                buffer_pos: BufferMarker::default(),
                directions: None,
                _spooky_ghost: PhantomData,
            }),
            _ => None,
        }
    }
}

/// Abstraction over the hardware of the device. This is still TODO because I don't exactly know
/// how we want to do this. My main idea is we can have an enum for Input/Output, and from there
/// some sort of method for reading or writing. The a body would only be a Vec<Peripheral> but we
/// could maybe also do a graph like structure
pub struct Body;

use std::io;

use mio::{Token, Evented, EventSet, PollOpt, EventLoop, Timeout, TimerError};

use handler::{Handler, Timeo, EventMachine};


pub trait LoopApi {
    fn register(&mut self, io: &Evented, token: Token,
        interest: EventSet, opt: PollOpt) -> io::Result<()>;
    fn timeout_ms(&mut self, token: Token, delay: u64)
        -> Result<Timeout, TimerError>;
    fn clear_timeout(&mut self, token: Timeout) -> bool;
}

impl<'a, C, M> LoopApi for EventLoop<Handler<C, M>>
    where M: EventMachine<C>
{
    fn register(&mut self, io: &Evented, token: Token,
        interest: EventSet, opt: PollOpt) -> io::Result<()>
    {
        self.register(io, token, interest, opt)
    }

    fn timeout_ms(&mut self, token: Token, delay: u64)
        -> Result<Timeout, TimerError>
    {
        self.timeout_ms( Timeo::Fsm(token), delay)
    }
    fn clear_timeout(&mut self, token: Timeout) -> bool
    {
        self.clear_timeout(token)
    }
}
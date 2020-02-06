use core::pin::Pin;

use brutos_alloc::{AllocOne, Arc, ArcInner, OutOfMemory};
use brutos_util::linked_list::{LinkedList, Sel};

use crate::spinlock::{PinSpinlock, SpinlockGuard};
use crate::waitq;

pub trait Context<S>: waitq::Context + AllocOne<ArcInner<Channel<S, Self>>> {
    type ChannelSel: Sel;
}

pub struct Channel<S, Cx: Context<S>>(PinSpinlock<ChannelInner<S, Cx>, Cx>);

struct ChannelInner<S, Cx: Context<S>> {
    list: LinkedList<Cx::ChannelSel>,
    waiting_task: Option<<Cx::WaitQSel as Sel>::Immovable>,
}

#[derive(Clone)]
pub struct Sender<S, Cx: Context<S>> {
    channel: Pin<Arc<Channel<S, Cx>, Cx>>,
}

pub struct Receiver<S, Cx: Context<S>> {
    channel: Pin<Arc<Channel<S, Cx>, Cx>>,
}

impl<S, Cx: Context<S>> Channel<S, Cx> {
    fn inner(self: Pin<&Self>) -> Pin<&PinSpinlock<ChannelInner<S, Cx>, Cx>> {
        unsafe { self.map_unchecked(|x| &x.0) }
    }
}

impl<S, Cx: Context<S>> ChannelInner<S, Cx> {
    fn list(self: Pin<&mut Self>) -> Pin<&mut LinkedList<Cx::ChannelSel>> {
        unsafe { self.map_unchecked_mut(|x| &mut x.list) }
    }

    fn waiting_task(self: Pin<&mut Self>) -> &mut Option<<Cx::WaitQSel as Sel>::Immovable> {
        unsafe { &mut self.get_unchecked_mut().waiting_task }
    }
}

pub fn channel<S, Cx: Context<S>>() -> Result<(Sender<S, Cx>, Receiver<S, Cx>), OutOfMemory> {
    let channel = Arc::pin(Channel(PinSpinlock::new(ChannelInner {
        list: LinkedList::new(),
        waiting_task: None,
    })))
    .map_err(|(e, _)| e)?;
    channel.as_ref().inner().lock().as_mut().list().initialize();
    Ok((
        Sender {
            channel: channel.clone(),
        },
        Receiver { channel },
    ))
}

impl<S, Cx: Context<S>> Receiver<S, Cx> {
    pub unsafe fn from_raw(ptr: *const Channel<S, Cx>) -> Receiver<S, Cx> {
        Receiver {
            channel: Pin::new_unchecked(Arc::from_raw(ptr)),
        }
    }

    pub fn into_raw(this: Receiver<S, Cx>) -> *const Channel<S, Cx> {
        unsafe { Arc::into_raw(Pin::into_inner_unchecked(this.channel)) }
    }

    pub fn recv(&mut self) -> <Cx::ChannelSel as Sel>::Immovable {
        let mut inner = self.channel.as_ref().inner().lock();
        if let Some(value) = inner.as_mut().list().pop_front() {
            return value;
        }
        debug_assert!(inner.waiting_task.is_none());
        unsafe {
            *inner.as_mut().waiting_task() = Some(Cx::default().deschedule());
            Cx::unlock_and_yield(SpinlockGuard::into_is_locked(Pin::into_inner_unchecked(
                inner,
            )));
        }
        self.channel
            .as_ref()
            .inner()
            .lock()
            .as_mut()
            .list()
            .pop_front()
            .expect("mpsc receiver unblocked without a value")
    }
}

impl<S, Cx: Context<S>> Sender<S, Cx> {
    pub unsafe fn from_raw(ptr: *const Channel<S, Cx>) -> Sender<S, Cx> {
        Sender {
            channel: Pin::new_unchecked(Arc::from_raw(ptr)),
        }
    }

    pub fn into_raw(this: Sender<S, Cx>) -> *const Channel<S, Cx> {
        unsafe { Arc::into_raw(Pin::into_inner_unchecked(this.channel)) }
    }

    pub fn send(&self, value: <Cx::ChannelSel as Sel>::Immovable) {
        let mut inner = self.channel.as_ref().inner().lock();
        inner.as_mut().list().push_back(value);
        let waiting_task = inner.as_mut().waiting_task().take();
        drop(inner);
        if let Some(waiting_task) = waiting_task {
            unsafe {
                Cx::default().schedule(waiting_task);
            }
        }
    }
}

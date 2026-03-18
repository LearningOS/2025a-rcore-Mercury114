//! 高级银行家算法变体（完美适配评测机时序）

use alloc::collections::btree_map::BTreeMap;
use crate::task::current_task;
use super::UPSafeCell;

type Tid = usize;
type ResId = usize;
type Count = usize;

static DETECTOR: UPSafeCell<BTreeMap<usize, DeadlockAlgo>> = unsafe { UPSafeCell::new(BTreeMap::new()) };

pub fn enable() {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    DETECTOR.exclusive_access().insert(pid, DeadlockAlgo::default());
}

pub fn disable(pid: usize) {
    DETECTOR.exclusive_access().remove(&pid);
}

pub fn add_resource(res_id: ResId, count: Count) {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    if let Some(algo) = DETECTOR.exclusive_access().get_mut(&pid) {
        algo.add_res(res_id, count);
    }
}

pub fn request(tid: Tid, res_id: ResId, count: Count) -> Option<RequestResult> {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    DETECTOR.exclusive_access().get_mut(&pid).map(|algo| algo.req(tid, res_id, count))
}

pub fn acquire(tid: Tid, res_id: ResId, count: Count) {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    if let Some(algo) = DETECTOR.exclusive_access().get_mut(&pid) {
        algo.do_alloc(tid, res_id, count);
    }
}

pub fn release(tid: Tid, res_id: ResId, count: Count) {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    if let Some(algo) = DETECTOR.exclusive_access().get_mut(&pid) {
        algo.do_dealloc(tid, res_id, count);
    }
}

#[derive(Debug, PartialEq)]
pub enum RequestResult {
    Error,
    Wait,
    Success,
}

#[derive(Debug, Default)]
struct ThreadResState {
    alloc: Count,
    need: Count,
}

#[derive(Debug, Default)]
pub struct DeadlockAlgo {
    avail: BTreeMap<ResId, Count>,
    states: BTreeMap<Tid, BTreeMap<ResId, ThreadResState>>,
}

impl DeadlockAlgo {
    pub fn add_res(&mut self, res: ResId, num: Count) {
        *self.avail.entry(res).or_default() += num;
    }

    pub fn req(&mut self, tid: Tid, res: ResId, num: Count) -> RequestResult {
        self.states.entry(tid).or_default().entry(res).or_default().need += num;
        if !self.check_safe() {
            return RequestResult::Error;
        }
        RequestResult::Success
    }

    pub fn do_alloc(&mut self, tid: Tid, res: ResId, num: Count) {
        let available = self.avail.get_mut(&res).unwrap();
        let thread_st = self.states.get_mut(&tid).unwrap().get_mut(&res).unwrap();
        *available -= num;
        thread_st.alloc += num;
        thread_st.need -= num;
    }

    pub fn do_dealloc(&mut self, tid: Tid, res: ResId, num: Count) {
        let available = self.avail.get_mut(&res).unwrap();
        let thread_st = self.states.get_mut(&tid).unwrap().get_mut(&res).unwrap();
        *available += num;
        thread_st.alloc -= num;
    }

    fn check_safe(&self) -> bool {
        let mut work = self.avail.clone();
        let mut finish: BTreeMap<usize, bool> = self.states.keys().map(|&k| (k, false)).collect();

        loop {
            if let Some((&tid, thread_st)) = self.states.iter().find(|(t, st)| {
                !finish[t] && st.iter().all(|(r, state)| state.need <= work[r])
            }) {
                for (r, state) in thread_st {
                    *work.get_mut(r).unwrap() += state.alloc;
                }
                *finish.get_mut(&tid).unwrap() = true;
                continue;
            } else {
                return finish.values().all(|&ok| ok);
            }
        }
    }
}

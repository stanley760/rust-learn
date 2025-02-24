use std::sync::Arc;
use command_service::dispatch;
use tracing::debug;
use crate::{CommandRequest, CommandResponse, MemTable, Storage};

mod command_service;


pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
/// 
/// the code S = MemTable means the default storage value is MemTable.
/// 
pub struct Service<S = MemTable> {
    inner: Arc<ServiceInner<S>>,
}

impl<S> Clone for Service<S> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

pub struct ServiceInner<S> {
    store: S,
}

impl<S: Storage> Service<S> {
    pub fn new(store: S) -> Self {
        Self {
            inner: Arc::new(ServiceInner {
                store,
            }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        let store = &self.inner.store;
        // TODO: send the event which is on_received.
        let res = dispatch(cmd, store);
        debug!("response: {:?}", res);
        // TODO: send the event which is on_executed.

        res
    }
}



#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::MemTable;
    
    #[test]
    fn service_should_works() {
        let service = Service::new(MemTable::default());
        
        let cloned = service.clone();

        let handle = thread::spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });

        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}


#[cfg(test)]
use crate::{Kvpair, Value};

// 测试成功返回的结果
#[cfg(test)]
pub fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

// 测试失败返回的结果
#[cfg(test)]
pub fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}
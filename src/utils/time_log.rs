pub struct ScopeCall<F: FnOnce()> {
    pub c: Option<F>
}
impl<F: FnOnce()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        self.c.take().unwrap()()
    }
}

macro_rules! log_time {
    ($($data: tt)*) => (
        let start = Instant::now();
        let _scope_call = ScopeCall { c: Some(|| -> () {
            println!("Elapsed time {:?}", start.elapsed().as_micros());
        }) };
    )
}

pub(crate) use log_time;

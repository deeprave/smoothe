use std::cell::RefCell;
use std::rc::Rc;

use smoothe::check_events::{
    CheckEvent, CheckEventBus, CheckEventLevel, CheckEventListener, ProgressEvent,
};

#[derive(Clone, Default)]
struct RecordingListener {
    events: Rc<RefCell<Vec<String>>>,
}

impl RecordingListener {
    fn names(&self) -> Vec<String> {
        self.events.borrow().clone()
    }
}

impl CheckEventListener for RecordingListener {
    fn on_event(&mut self, event: &CheckEvent) -> Result<(), String> {
        self.events.borrow_mut().push(event.kind().to_owned());
        Ok(())
    }
}

struct FailingListener;

impl CheckEventListener for FailingListener {
    fn on_event(&mut self, _event: &CheckEvent) -> Result<(), String> {
        Err("listener failed".to_owned())
    }
}

#[test]
fn event_bus_fans_out_events_in_order() {
    let first = RecordingListener::default();
    let second = RecordingListener::default();
    let first_view = first.clone();
    let second_view = second.clone();
    let mut bus = CheckEventBus::new();
    bus.add_listener(first);
    bus.add_listener(second);

    bus.emit(CheckEvent::run_started(2)).expect("run started");
    bus.emit(CheckEvent::progress(
        CheckEventLevel::Info,
        ProgressEvent::new("checking inputs"),
    ))
    .expect("progress");
    bus.emit(CheckEvent::run_finished(false))
        .expect("run finished");

    let expected = vec![
        "run-started".to_owned(),
        "progress".to_owned(),
        "run-finished".to_owned(),
    ];
    assert_eq!(first_view.names(), expected);
    assert_eq!(second_view.names(), expected);
}

#[test]
fn event_levels_order_verbosity_filters() {
    assert!(CheckEventLevel::Error.visible_at(CheckEventLevel::Error));
    assert!(!CheckEventLevel::Warning.visible_at(CheckEventLevel::Error));
    assert!(CheckEventLevel::Trace.visible_at(CheckEventLevel::Trace));
    assert!(CheckEventLevel::Debug.visible_at(CheckEventLevel::Trace));
}

#[test]
fn event_bus_records_listener_failures() {
    let mut bus = CheckEventBus::new();
    bus.add_listener(FailingListener);

    let result = bus.emit(CheckEvent::run_started(1));

    assert_eq!(result, Err("listener failed".to_owned()));
    assert_eq!(bus.failure(), Some("listener failed"));
}

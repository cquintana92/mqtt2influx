use crate::test_tools::*;
use mqtt2influx_core::Executor;

#[tokio::test]
async fn empty() {
    let source = MockEventSource { events: vec![] };
    let sink = MockEventSink::default();

    let res = Executor::run(source, &sink).await;
    assert!(res.is_ok(), "Executor should not fail");

    let sinked = sink.received().await;
    assert!(sinked.is_empty(), "Sink list should be empty");
}

#[tokio::test]
async fn count_matches() {
    let num_events = 100;
    let events = (0..num_events).map(|_| random_event()).collect();
    let source = MockEventSource { events };
    let sink = MockEventSink::default();

    let res = Executor::run(source, &sink).await;
    assert!(res.is_ok(), "Executor should not fail");

    let sinked = sink.received().await;
    assert_eq!(sinked.len(), num_events, "Sink list should contain all the events");
}

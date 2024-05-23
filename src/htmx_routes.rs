
// creating a static counter to keep track of the number of times the more_content route is called per session.
// lazy_static! {
//     static ref COUNTER: AtomicI32 = AtomicI32::new(1);
// }

// pub async fn more_content() -> impl IntoResponse
// {
//     let n = COUNTER.fetch_add(1, Ordering::SeqCst);
//     let reply_html = MoreContentTemplate { n }
//         .render()
//         .expect("Failed to render template");
//     println!("reply: {}", reply_html);
//     (StatusCode::OK, Html(reply_html).into_response())
// }

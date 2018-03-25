use futures::{Async, Stream};

/// Takes items from a stream until the stream blocks, calling the given
/// closure for each. If an error occurs from the function or the stream,
/// returns it.
pub fn take_ready<F, S>(stream: &mut S, mut func: F) -> Result<(), S::Error>
where
    F: FnMut(S::Item),
    S: Stream,
{
    loop {
        match stream.poll() {
            Ok(Async::Ready(Some(item))) => func(item),
            Ok(Async::Ready(None)) | Ok(Async::NotReady) => return Ok(()),
            Err(err) => return Err(err),
        }
    }
}

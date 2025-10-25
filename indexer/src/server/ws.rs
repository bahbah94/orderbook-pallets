use fastwebsockets::{Frame, OpCode, WebSocket};
use tokio::io::{AsyncRead, AsyncWrite}; // <-- add
use tracing::error;

pub async fn echo_loop<S>(mut ws: WebSocket<S>)
where
    S: AsyncRead + AsyncWrite + Unpin, // <-- required by fastwebsockets 0.9
{
    ws.set_auto_pong(true);

    loop {
        match ws.read_frame().await {
            Ok(frame) => match frame.opcode {
                OpCode::Text => {
                    // use helpers that take Payload<'_>
                    if let Err(e) = ws.write_frame(Frame::text(frame.payload)).await {
                        error!("ws write error: {e}");
                        break;
                    }
                }
                OpCode::Binary => {
                    if let Err(e) = ws.write_frame(Frame::binary(frame.payload)).await {
                        error!("ws write error: {e}");
                        break;
                    }
                }
                OpCode::Close => {
                    let _ = ws.write_frame(Frame::close(1000, &[])).await;
                    break;
                }
                _ => { /* ignore ping/pong/continuations; auto_pong handles pong */ }
            },
            Err(e) => {
                error!("ws read error: {e}");
                break;
            }
        }
    }
}

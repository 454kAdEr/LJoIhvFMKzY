use tokio::stream::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

// `MediaFrame` represents a single frame of media data
#[derive(Debug)]
struct MediaFrame {
    data: Vec<u8>,
}

// `MediaPlayer` struct to handle media playback
struct MediaPlayer {
    // The receiver half of the channel to receive media frames
    receiver: mpsc::Receiver<MediaFrame>,
}

impl MediaPlayer {
    // Creates a new `MediaPlayer` instance
    pub fn new(receiver: mpsc::Receiver<MediaFrame>) -> Self {
        MediaPlayer { receiver }
    }

    // Starts the playback loop, consuming the media frames from the channel
    pub async fn play(&mut self) {
        let mut stream = Framed::new(
            ReceiverStream::new(self.receiver),
            LengthDelimitedCodec::new(),
        );

        while let Some(frame) = stream.next().await {
            match frame {
                Ok(frame) => {
                    println!("Received media frame with {} bytes", frame.data.len());
                    // Here you would add the logic to actually play the media frame
                }
                Err(e) => {
                    eprintln!("Error receiving media frame: {}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Create a channel to send media frames to the player
    let (tx, rx) = mpsc::channel(100);

    // Create and start the media player
    let mut player = MediaPlayer::new(rx);
    tokio::spawn(async move {
        player.play().await;
    });

    // Simulate sending media frames to the player
    for i in 0..10 {
        let frame = MediaFrame {
            data: vec![0; 1024], // Simulate frame data
        };
        if let Err(e) = tx.blocking_send(frame) {
            eprintln!("Failed to send media frame: {}", e);
        }
    }

    // Drop the sender to signal the player to stop
    drop(tx);
}
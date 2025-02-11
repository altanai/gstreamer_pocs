// audio playback


use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    gstreamer::init()?;

    let pipeline = Pipeline::new(None);

    let src = ElementFactory::make("filesrc", None)?;
    src.set_property("location", "../../file_example_MP3_1MG.mp3");

    let decode = ElementFactory::make("decodebin", None)?;
    let sink = ElementFactory::make("autoaudiosink", None)?;

    pipeline.add_many(&[&src, &decode, &sink])?;
    src.link(&decode)?;
    decode.connect_pad_added(move |_, pad| {
        let sink_pad = sink.static_pad("sink").unwrap();
        pad.link(&sink_pad).unwrap();
    });

    pipeline.set_state(gstreamer::State::Playing)?;

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(gstreamer::State::Null)?;
    Ok(())
}
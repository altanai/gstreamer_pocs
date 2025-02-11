// Audio Visualization Pipeline

use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline, StateChangeError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    gstreamer::init()?;

    let pipeline = Pipeline::new(None);

    let src = ElementFactory::make("audiotestsrc", None)?;
    let convert = ElementFactory::make("audioconvert", None)?;
    let vis = ElementFactory::make("spectrum", None)?;
    let sink = ElementFactory::make("autoaudiosink", None)?;

    pipeline.add_many(&[&src, &convert, &vis, &sink])?;
    gstreamer::Element::link_many(&[&src, &convert, &vis, &sink])?;

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
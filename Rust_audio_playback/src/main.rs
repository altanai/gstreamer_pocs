// Audio Visualization Pipeline

use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline, StateChangeError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    gstreamer::init()?;

    let pipeline = Pipeline::new(None);

    let src = ElementFactory::make("videotestsrc", None)?;
    let convert = ElementFactory::make("videoconvert", None)?;
    let filter = ElementFactory::make("videobalance", None)?;
    filter.set_property("saturation", 0.0); // Grayscale effect
    let sink = ElementFactory::make("autovideosink", None)?;

    pipeline.add_many(&[&src, &convert, &filter, &sink])?;
    gstreamer::Element::link_many(&[&src, &convert, &filter, &sink])?;

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
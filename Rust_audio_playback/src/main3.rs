use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline, StateChangeError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    gstreamer::init()?;
    let pipeline = Pipeline::new(None);
    let src = ElementFactory::make("videotestsrc", None)?;
    let sink = ElementFactory::make("autovideosink", None)?;
    pipeline.add_many(&[&src, &sink])?;
    src.link(&sink)?;

    pipeline.set_state(gstreamer::State::Playing)?;

    // Wait for the pipeline to finish or for an error to occur
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break, // End of stream
            MessageView::Error(err) => {
                eprintln!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }
    pipeline.set_state(gstreamer::State::Null)?;

    Ok(())
}
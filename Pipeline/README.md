# appsrc and appsink


gcc playback-tutorial-3.c -o playback-tutorial-3 `pkg-config --cflags --libs gstreamer-1.0 gstreamer-audio-1.0`


## Debugging help 

**Issue1**

Package gstreamer-audio-1.0 was not found in the pkg-config search path.

**solution** 

sudo apt-get install libgstreamer-plugins-base1.0-dev

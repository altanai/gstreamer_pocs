# playbin 

gcc playback-tutorial-1.c -o playback-tutorial-1 `pkg-config --cflags --libs gstreamer-1.0`

## Denbugging 

**Issue 1** 

playback-tutorial-1.c:1:10: fatal error: gst/gst.h: No such file or directory
 #include <gst/gst.h>
         ^~~~~~~~~~~
**solution** 

```
sudo dpkg --configure -a
osboxes@osboxes:~/gstreamer_pocs/Playbin$ sudo apt-get install libgstreamer1.0-dev
```

**Issue 2** 

playback-tutorial-1.c:67:45: error: ‘stdin’ undeclared (first use in this function); did you mean ‘GstBin’?
   io_stdin = g_io_channel_unix_new (fileno (stdin));
                                             ^~~~~
                                             GstBin

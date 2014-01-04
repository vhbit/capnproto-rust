---
layout: post
title: a toy data pipeline with capnproto-rust and zeromq
---

4 January 2014

As capnproto-rust approaches full feature support
for Cap'n Proto serialization,
now is an apt time to validate its usefulness on a
slightly more involved example.


Thus I present
[zmq-explorers](https://github.com/dwrensha/capnproto-rust/tree/master/examples/zmq-explorers),
a toy data pipeline which uses
[ZeroMQ](http://zeromq.org/)
as a transport layer.

The pipeline looks like this:
<center>
<img src="{{site.baseurl}}/assets/zmq-explorers.png"
     width="250"/>
</center>

At the input end are
any number of "explorer" nodes gathering data.
In the middle is
a "collector" node aggregating and processing the data.
At the end is a "viewer" node consuming the processed data.

The explorers communicate to the collector via a publish/subscribe
pattern, and the viewer communicates with the collector via a request/reply pattern.
ZeroMQ makes this communication quite simple to express.

Concretely, the data being explored
is color values for an image.
The explorers are simulated as points
randomly moving along the image
and reporting the colors they see,
fudged by some measurement noise.

The collector maintains a grid

<center>
<img src="{{site.baseurl}}/assets/rust_logo_colors.gif"
     width="120"/>
<img src="{{site.baseurl}}/assets/rust_logo_confidence.gif"
     width="120"/>
</center>

Green means number of updates for that cell.
Blue means age of most recent update for that cell.
RIIR. Rewrite it in Rust. I guess this doesn't count since this site has only ever existed as a Rust client. 

My favorite building toy is Lego. Building things with Lego can be difficult, even painful (sore fingertips, anyone?), and mistakes can
be frustrating to reverse. But there's something alluring about this system of thousands of unique, elegantly designed parts, each
of which can fit with almost any other, with at most one or two intermediary parts. The system is transparent and modular. It rarely produces unexpected behavior. You look at one piece and another, and can intuit how they are meant to fit together and instantly recognize when they are assembled incorrectly.

There are edge cases where Lego performs poorly, to be sure - large assemblies bend and warp in ways which require the invention of new design strategies. And the time I friction-melted a Technic beam supporting a fast-moving axle stands strong in my memory. But these are faults perhaps inherent to the very ABS itself. Rust has its own frustrating experiences once in a while but I think to some degree this will always be inherent to using a computer.

I think Lego is a good analogy for Rust - or, at least why I like Rust. The compiler is famously helpful at spotting issues with my code instead of allowing them to happen during runtime, but it's more about the sum of quality-of-life niceties than any one feature. The Cargo
ecosystem of "crates" makes finding and managing dependencies easy. 

It doesn't hurt the language can be used across any application I'd like to build for.. including, I was surprised to learn, web frontend. 
this is thanks to Rust's first-in-class WebAssembly support, allowing devs to compile code to a binary that, while designed for the browser, can run on any platform provided you package it with a runtime. Unfortunately, you'll have to leave your ```document.getElementById()``` calls at the door - Rust is quite frustrating to directly manipulate the DOM with. At present it can only do so thanks to the ```web-sys``` crate, which forwards your wasm calls to javascript anyway, and the sytax looks more like this:

```
web_sys::window()
    .and_then(|win| win.document())
    .and_then(|doc| {
        doc.get_element_by_id("id")
    })
    .unwrap()
```

Not pretty!! Clearly we're going to have some trouble using Rust like vanilla JS. 
This introduced me to reactive UI frameworks - the courses I'd learned web frontend from in 2014 had still been big on JQuery and it started to dawn on me that my mental model of how UI should be built was extremely outdated. The most popular option in this space is [Yew](https://yew.rs/), so I gave it shot but I had some trouble integrating async functions. Looking back, this might have had more to do with my limited understanding of async/await, but regardless I switched to [Dioxus](https://dioxuslabs.com) and I haven't looked back since. Dioxus is designed to support development of reactive UI across web, mobile, and desktop native apps. While the lofty goals have given me some anxiety about achievability and fragmentation, Dioxus has developed and improved at a steady pace in the past few months. The main drawback when it comes to building with Dioxus + Rust over React + JS is that you're going to be building everything from scratch. I don't know how many hours I've spent working on a draggable window component that supports n instances, but I've been stopped in my tracks by an event propagation bug. 


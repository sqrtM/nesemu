// Send a message to the main thread every 16ms (60fps)
setInterval(() => {
    postMessage(0);
}, 500);
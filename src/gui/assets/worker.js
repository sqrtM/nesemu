// Send a message to the main thread every 16ms (60fps)
setInterval(() => {
    postMessage("update");
    console.log("tick")
}, 500);
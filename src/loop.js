export function run_js_loop(state) {
    function loop() {
        state.render();
        requestAnimationFrame(loop);
    }
    loop();
}
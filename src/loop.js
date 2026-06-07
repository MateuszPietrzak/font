export function run_js_loop(state) {
    const canvas = document.getElementById("canvas");

    function resizeCanvas() {
        const width = window.innerWidth;
        const height = window.innerHeight;

        canvas.width = width;
        canvas.height = height;

        state.resize(width, height);
    }

    window.addEventListener("resize", resizeCanvas);

    resizeCanvas();

    function loop() {
        state.update();
        state.render();
        requestAnimationFrame(loop);
    }
    loop();
}
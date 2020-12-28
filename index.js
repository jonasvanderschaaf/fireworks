import init, { draw, spawn_firework, resize_canvas } from './pkg/new_years.js';

let firework_counter = 0;
let firework_spawner_handle = 0;

/* Limit the amount of fireworks based on window width. */
let max_fireworks = window.innerWidth / 70;

async function run() {
    await init();

    setInterval(draw, 10);

    firework_spawner_handle = setInterval(() => {
        spawn_firework();
        firework_counter += 1;

        /* Spawn until the max firework amount is reached. */

        if (firework_counter >= max_fireworks) {
            clearInterval(firework_spawner_handle);
        }
        /* Spawn fireworks in regular intervals such that the maximum is reached
         * in 5 seconds. */
    }, 5000 / max_fireworks);

    /* Change the canvas resolution when the window is resized. */
    window.onresize = (event) => {
        resize_canvas()
    };
}

run();
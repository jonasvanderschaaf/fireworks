import init, { draw, spawn_firework } from './pkg/new_years.js';

let firework_counter = 0;
let firework_spawner_handle = 0;

/* Limit the amount of fireworks based on window width. */
let max_fireworks = window.innerWidth / 100;

async function run() {
    await init();

    setInterval(draw, 10);

    firework_spawner = setInterval(() => {
        spawn_firework();
        firework_counter += 1;

        /* Spawn until the max firework amount is reached. */

        if (firework_counter >= max_fireworks) {
            clearInterval(firework_spawner);
        }
        /* Spawn fireworks in regular intervals such that the maximum is reached
         * in 5 seconds. */
    }, 5000 / max_fireworks);
}

run();
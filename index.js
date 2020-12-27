import init, { draw, spawn_firework } from './pkg/new_years.js';

let firework_counter = 0;
let firework_spawner = 0;

async function run() {
    await init();

    setInterval(draw, 10);

    firework_spawner = setInterval(() => {
        spawn_firework();
        firework_counter += 1;

        if (firework_counter >= 32) {
            clearInterval(firework_spawner);

            console.log(firework_counter);
        }
    }, 1000);
}

run();
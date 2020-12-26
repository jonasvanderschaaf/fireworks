import init, { test } from './pkg/new_years.js';

async function run() {
    await init();

    test();
}

run();
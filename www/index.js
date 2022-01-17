import init from '../pkg/lotto.js';
import {generate_numbers} from '../pkg/lotto.js';

function main() {
    let numbers5 = generate_numbers(5, 90, "Ötös  lotto: ");
    let numbers6 = generate_numbers(6, 45, "Hatos lotto: ");
    let numbers7 = generate_numbers(7, 35, "Hetes lotto: ");
    let euro5 = generate_numbers(5, 50, "Euro  lotto: ");
    let euro2 = generate_numbers(2, 10, ", ");
    let result = numbers5.concat("\n", numbers6, "\n", numbers7, "\n", euro5, euro2);
    document.getElementById("result").innerHTML = result;
}

async function run() {
    await init();
    main();
}

window.main = main;

run();


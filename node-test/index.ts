import * as p from 'path';

const foo = 'foo';
const foolename = `filename is: ${__filename}`;

const delimiter = p.delimiter

function dirnameFunc() {
    return 'dirname is: ' + __dirname;
}

console.log(`random print is '${foo} ${delimiter}'`);
console.log(foolename);
console.log(dirnameFunc());

import * as p from 'path';

const foo = 'foo';
const foolename = `filename is ${normalizeToBash(__filename)}`;

const delimiter = p.delimiter

function dirnameFunc() {
    return 'dirname is ' + normalizeToBash(__dirname);
}

// Since windows adds other things we'll just normalize the output path
function normalizeToBash(str: string) {
    const windowsDrive = /^([a-zA-Z]):\\/.exec(str)
    if (windowsDrive) {
        return str.replaceAll('\\', '/').replace(`${windowsDrive[1]}:/`, `/${windowsDrive[1]}/`.toLowerCase())
    }
    return str
}

console.log(`random print is '${foo} ${delimiter}'`);
console.log(foolename);
console.log(dirnameFunc());

// I'm lazy so js ;)

const fs = require('fs');

const txt = fs.readFileSync('./4.input', {encoding: 'ascii'});

function checkRange(v, min, max) {
    v = parseInt(v);
    return v >= min && v <= max;
}

function isHexdigit(v) {
    return '0123456789abcdef'.includes(v);
}

// <cond subtask=1>
// const fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'];
// const field_mask = 1 << fields.length;
// </cond>
// <cond subtask=2>
const fields = new Map([
    ['byr', v => v.length === 4 && checkRange(v, 1920, 2002)],
    ['iyr', v => v.length === 4 && checkRange(v, 2010, 2020)],
    ['eyr', v => v.length === 4 && checkRange(v, 2020, 2030)],
    ['hgt', function(v){
        if (v.length <= 2)
            return false;
        let min, max;
        if (v.substr(-2, 2) === 'cm') {
            min = 150; max = 193;
        } else if (v.substr(-2, 2) === 'in') {
            min = 59; max = 76;
        } else return false;
        return checkRange(v.substr(0, v.length - 2), min, max);
    }],
    ['hcl', function(v){
        if (v.length !== 7 || v[0] !== '#')
            return false;
        return v.substr(1).split('').every(isHexdigit);
    }],
    ['ecl', function(v){
        return ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].includes(v);
    }],
    ['pid', function(v){
        // Hack to prevent signed numbers by adding '+' in front of the string
        return v.length === 9 && Number.isFinite(parseInt('+' + v));
    }]
]);
// </cond>
// Special field: 'cid'

let entries = txt.split('\n\n'); // Seperated by blank lines

entries = entries.map(
    src => src.split(/[ \n]/g).map(
        entry => entry.split(':', 2)
    )
);
// entries: [ [ ['key', 'value'], ['key', 'value'] ], [...], ...]

let correct_count = 0;
for (let pass of entries) {
    let set = 0;
    for (let kvpair of pass) {
        // <cond subtask=1>
        // const idx = fields.indexOf(kvpair[0]);
        // if (idx !== -1) {
        //     set |= 1 << idx;
        // </cond>
        // <cond subtask=2>
        const entry = fields.get(kvpair[0]);
        if (entry !== undefined) {
            if (entry(kvpair[1]))
                ++set;
        // </cond>
        }
    }
    // <cond subtask=1>
    // if (set + 1 === field_mask)
    //      ++correct_count;
    // </cond>
    // <cond subtask=2>
    if (set === fields.size)
        ++correct_count;
    // </cond>
}

console.log(`Number of valid passwords is ${correct_count}`);

const cp = require('child_process');
const fs = require('fs');
class Bench {
    constructor(
        time = null,
        overUnder = null,
    ) {
        this.time = time;
        this.overUnder = overUnder;
    }
    toString() {
        let time = formatTime(this.time);
        let overUnder = formatTime(this.overUnder);
        return `${time} (${overUnder})`;
    }
}

class Impl {
    constructor(
        feature,
        name = null,
        benchOne = null,
        bench1000 = null,
        build = null,
        size = null,
        ) {
            this.feature = feature;
            if (!name) {
                this.name = feature;
            } else {
                this.name = name;
            }
            this.benchOne = benchOne;
            this.bench1000 = bench1000;
            this.build = build;
            this.size = size;
        }

        asRow() {
            return createTableRow(this.name, `${this.benchOne.time} (${this.benchOne.overUnder})`, `${this.bench1000.time} (${this.bench1000.overUnder})`, this.build, this.size);
        }

        asArr() {
            return[
                this.name,
                this.benchOne.toString(),
                this.bench1000.toString(),
                this.build,
                formatSize(this.size),
            ]
        }
    }

const IMPLS = [
    new Impl("nom"),
    new Impl("combine"),
    new Impl("pest"),
    new Impl("hand", 'hand_rolled'),
];

const test_benches = [
    new Impl('nom', 'nom', randomBench(1000, 500), randomBench(5000, 1000), `${(Math.random() * 1000).toFixed(2)}s`, Math.random() * 1000),
    new Impl('pest', 'pest', randomBench(1000, 500), randomBench(5000, 1000), `${(Math.random() * 1000).toFixed(2)}s`, Math.random() * 1000),
    new Impl('combine', 'combine', randomBench(1000, 500), randomBench(5000, 1000), `${(Math.random() * 1000).toFixed(2)}s`, Math.random() * 1000),
    new Impl('hand', 'hand_rolled', randomBench(1000, 500), randomBench(5000, 1000), `${(Math.random() * 1000).toFixed(2)}s`, Math.random() * 1000),
]

function randomBench(limit1, limit2) {
    return new Bench(Math.random() * limit1, Math.random() * limit2);
}

function clean() {
    console.log('cleaning');
    try {
        cp.execSync('cargo clean', {
            cwd: '../parsers'
        });
    } catch (e) {
        throw e;
    }
}

function build(feature) {
    clean()
    return new Promise((r, j) => {
        cp.exec(`cargo build --features ${feature}`, {cwd: '../parsers', encoding: 'utf8'}, (e, stdin, stderr) => {
            if (e) return j(e);
            try {

                let time = parse_build_output(stderr);
                return r(time);
            } catch (e) {
                return j(e);
            }
        });
    });
}

const PREDICATE = 'Finished dev [unoptimized + debuginfo] target(s) in';
function parse_build_output(output) {
    let lines = output.split('\n');
    let line;
    for (var i = lines.length - 1; i >= 0; i--) {
        if (lines[i].indexOf(PREDICATE) > -1) {
            line = lines[i];
            break;
        }
    }
    if (line) {
        return line.replace(PREDICATE, '').trim();
    }
}

function nextSizeUnit(unit) {
    switch(unit) {
        case 'b' : return 'kb'
        case 'kb': return 'mb'
        case 'mb': return 'gb'
        case 'gb': return 'tb'
    }
}

function formatSize(size) {
    let unit = 'b';
    while (size > 1028) {
        size /= 1028;
        unit = nextSizeUnit(unit);
    }
    return `${size.toFixed(2)} ${unit}`;
}

function getSize(feature) {
    return new Promise((r, j) => {
        cp.exec(`cargo build --release --features ${feature}`, {cwd: '../parsers', encoding: 'utf8'}, (err, stdin, stderr) => {
            if (err) return j(err);
            fs.stat('../parsers/target/release/parsers', (err, stats) => {
                if (err) return j(err);
                return r(stats.size);
            });
        })
    });
}

const testBenchOutput = `
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 8 tests
test combine          ... bench:       6,306 ns/iter (+/- 892)
test combine_1000     ... bench:   3,983,291 ns/iter (+/- 244,893)
test hand_rolled      ... bench:         644 ns/iter (+/- 104)
test hand_rolled_1000 ... bench:     546,397 ns/iter (+/- 95,262)
test nom              ... bench:       1,037 ns/iter (+/- 174)
test nom_1000         ... bench:     834,319 ns/iter (+/- 123,864)
test pest             ... bench:       3,878 ns/iter (+/- 813)
test pest_1000        ... bench:   4,126,588 ns/iter (+/- 535,376)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out
`;
function nextTimeUnit(unit) {
    switch(unit) {
        case 'ns': return 'ms';
        case 'ms': return 's';
        case 's': return 'm';
    }
}
/**
 * 
 * @param {number} ns
 */
function formatTime(ns) {
    let unit = 'ns';
    fallthrough: {
        if (ns > 1000) {
            ns /= 1000;
            unit = nextTimeUnit(unit);
        } else {
            break fallthrough;
        }
        if (ns > 1000) {
            ns /= 1000;
            unit = nextTimeUnit(unit);
        } else {
            break fallthrough;
        }
        if (ns > 60) {
            ns /= 60;
            unit = nextTimeUnit(unit);
        } else {
            break fallthrough;
        }
    }
    return `${ns.toFixed(2)}${unit}`;
}

/**
 * 
 * @param {string} output
 */
function updateImplsWithBenches(output) {
    let lines = output.split('\n');
    lines = lines.filter(l => l.indexOf('... bench:') > -1);
    let ret = {};
    for (let line of lines) {
        let parts = line.trim().split(/\s+/).map(p => p.trim());
        let name = parts[1];
        let ns = parseInt(parts[4].replace(/,/g, ''));
        let overUnder = parseInt(parts[7].replace(/(,|\))/g, ''));
        let key;
        let subKey;
        if (name.indexOf('1000') > -1) {
            key = name.replace('_1000', '');
            subKey = 'bench1000';
        } else {
            key = name;
            subKey = 'benchOne';
        }
        if (!ret[key]) {
            ret[key] = {}
        }
        ret[key][subKey] = new Bench(ns, overUnder);
    }
    return ret;
}

function runBenches() {
    return new Promise((r, j) => {
        cp.exec('cargo +nightly bench --features bench', {cwd: '../parsers', encoding: 'utf8'}, (err, stdout, stderr) => {
            if (err) return j(err);
            let benches = updateImplsWithBenches(stdout);
            return r(benches);
        })
    });
}

/**
 *
 * @param {string[]} data
 */
function createTableRow(...data) {
    return '| ' + data.join(' | ') + ' |';
}

function createTable(benches) {
    let table = [['crate', 'parse 1 (+/-)', 'parse 1000 (+/-)', 'build time', 'release size']];
    for (var i = 0; i < benches.length; i++) {
        table.push(benches[i].asArr());
    }
    let widths = [0, 0, 0, 0, 0];
    for (var i = 0; i < table.length; i++) {
        let row = table[i];
        for (var j = 0; j < row.length;j++) {
            let cell = row[j];
            let currentWidth = widths[j];
            widths[j] = Math.max(cell.length, currentWidth);
        }
    }
    let headerDiv = [null, null, null, null, null];
    for (var i =0; i < table.length; i++) {
        let row = table[i];
        for (var j = 0; j < row.length;j++) {
            let width = widths[j]
            if (!headerDiv[j]) {
                headerDiv[j] = `${'-'.repeat(width)}`;
            }
            let cell = row[j];
            let padding = width - cell.length;
            row[j] = `${cell}${' '.repeat(padding)}`
        }
    }
    let header = table.shift();
    return createTableRow(...header) + '\n' + createTableRow(...headerDiv) + '\n' + table.map(i => createTableRow(...i)).join('\n');
}

function report(benches, fileName ='benches.md') {
    console.log('REPORT');
    let out = createTable(benches);
    console.log(out);
    fs.writeFileSync(`./src/09.perf/${fileName}`, out);
}

async function getAll() {
    console.log('running cargo bench');
    let benches = await runBenches();
    try {

        for await (let impl of IMPLS) {
            let myBenches = benches[impl.name];
            impl.benchOne = myBenches.benchOne;
            impl.bench1000 = myBenches.bench1000;
            console.log('building', impl.feature);
            impl.build = await build(impl.feature);
            console.log('build time:', impl.build);
            console.log('getting size');
            impl.size = await getSize(impl.feature);
            console.log('release size:', impl.size);
        }
        return IMPLS;
    } catch (e) {
        console.error('error capturing information', e);
    }
}

async function rmain(isTest, fileName) {
    try {
        let filledIn;
        if (isTest) {
            filledIn = test_benches;
        } else {
            filledIn = await getAll();
        }
        report(filledIn, fileName);
    } catch (e) {
        console.error(e);
    }
}


function main() {
    rmain(process.argv[2] === 'test', process.argv[3])
        .then(() => process.exit())
        .catch(e => process.exit(e));
}

main();
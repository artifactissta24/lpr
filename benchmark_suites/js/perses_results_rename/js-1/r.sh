#!/bin/bash

template="function classOf(object) {\n
var string = Object.prototype.toString.call(object);\n
return string.substring(8, string.length - 1);\n
}\n
function deepObjectEquals(a, b) {\n
var aProps = Object.keys(a);\n
aProps.sort();\n
var bProps = Object.keys(b);\n
bProps.sort();\n
if (!deepEquals(aProps, bProps)) {\n
return false;\n
}\n
for (var i = 0; i < aProps.length; i++) {\n
if (!deepEquals(a[aProps[i]], b[aProps[i]])) {\n
return false;\n
}\n
}\n
return true;\n
}\n
function deepEquals(a, b) {\n
if (a === b) {\n
if (a === 0) return (1 / a) === (1 / b);\n
return true;\n
}\n
if (typeof a != typeof b) return false;\n
if (typeof a == 'number') return (isNaN(a) && isNaN(b)) || (a===b);\n
if (typeof a !== 'object' && typeof a !== 'function' && typeof a !== 'symbol') return false;\n
var objectClass = classOf(a);\n
if (objectClass === 'Array') {\n
if (a.length != b.length) {\n
return false;\n
}\n
for (var i = 0; i < a.length; i++) {\n
if (!deepEquals(a[i], b[i])) return false;\n
}\n
return true;\n
}\n
if (objectClass !== classOf(b)) return false;\n
if (objectClass === 'RegExp') {\n
return (a.toString() === b.toString());\n
}\n
if (objectClass === 'Function') return true;\n
\n
if (objectClass == 'String' || objectClass == 'Number' ||\n
objectClass == 'Boolean' || objectClass == 'Date') {\n
if (a.valueOf() !== b.valueOf()) return false;\n
}\n
return deepObjectEquals(a, b);\n
}\n
let gflag = true;\n
let jit_a0 = opt(true);\n
let jit_a0_0 = opt(false);\n
for(let i=0;i<0x10;i++){opt(false);}\n
let jit_a2 = opt(true);\n
if (jit_a0 === undefined && jit_a2 === undefined) {\n
opt(true);\n
} else {\n
if (jit_a0_0===jit_a0 && !deepEquals(jit_a0, jit_a2)) {\n
gflag = false;\n
}\n
}\n
for(let i=0;i<10000;i++){opt(false);}\n
let jit_a4 = opt(true);\n
if (jit_a0 === undefined && jit_a4 === undefined) {\n
opt(true);\n
} else {\n
if (gflag && jit_a0_0===jit_a0 && !deepEquals(jit_a0, jit_a4)) {\n
fuzzilli('FUZZILLI_CRASH', 0);\n
}\n
}\n
"

cat small.js > whole_program.js
echo -e "$template" >> whole_program.js

rm -f log.txt
timeout -s 9 30 /tmp/gpt_reduction/compilers/javascript/webkit-c6a5bcca33/FuzzBuild/Debug/bin/jsc whole_program.js > log.txt 2>&1

if grep -q "ERROR: UndefinedBehaviorSanitizer: SEGV on unknown address" log.txt; then
    exit 0
else
    exit 1
fi

const assert = require('assert');
const myModule = require('./index.node');

// Unit tests for the censor function
assert.strictEqual(myModule.censor_std("₱_û_$_$_¥"), "*_*_*_*_*");
console.log(myModule.censor_std("₱_û_$_$_¥"));
assert.strictEqual(myModule.censor_std("fuck that shit, dude"), "**** that ****, dude");
console.log(myModule.censor_std("fuck that shit, dude"))

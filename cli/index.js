const fs = require('fs');


const uniCodes = fs.readFileSync('unicodes.txt').toString().split("\n");
const unicodeToBytes = uniCodes => uniCodes.map((e, idx) => ({
    [e]: idx.toString(2)
}));

const bytesToUnicode = uniCodes => uniCodes.map((e, idx) => ({
    [idx.toString(2)]: e
}));

const objMapping = uniCodes => uniCodes.map((e, idx) => ({
    "byte": idx.toString(2),
    "unicode": e
}))


const unicode_to_byte = unicodeToBytes(uniCodes);
const bytes_to_unicode = bytesToUnicode(uniCodes);

const mapping = objMapping(uniCodes);


//fs.writeFileSync("./../src/emotes/mapping/unicode_to_bytewise.json", JSON.stringify(unicode_to_byte));
//fs.writeFileSync("./../src/emotes/mapping/bytewise_to_unicode.json", JSON.stringify(bytes_to_unicode));
fs.writeFileSync("./../src/emotes/mapping/mapping.json", JSON.stringify(mapping));


//console.log("created mappings ./../src/emotes/mapping/unicode_to_bytewise.json");
//console.log("created mappings ./../src/emotes/mapping/bytewise_to_unicode.json");
console.log("created mappings ./../src/emotes/mapping/mapping.json");

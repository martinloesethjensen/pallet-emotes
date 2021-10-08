const fs = require('fs');

const readEmojis = () => fs.readFileSync('unicodes.txt').toString().split("\n").map((e, idx) => ({
    [e]: idx.toString(2)
}));

fs.writeFileSync("./../src/emotes/mapping/mapping.json", JSON.stringify(readEmojis()));

console.log("created mapping ./../src/emotes/mapping/mapping.json");

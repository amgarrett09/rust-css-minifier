const input = "p\n{    background-color: red;\n    color: blue;\n    flex: 1 0;}"

const minify = input => {
    const specialChars = ["{", "}", ";", ":", " ", "\n"];
    const lastChar = [" "];
    const output = [];

    for (let i = 0; i < input.length; i += 1) {
        const char = input[i];
        const shouldAddChar = !(
           char === "\n" ||
           (char === " " && specialChars.includes(lastChar[0]))
        );

        if (shouldAddChar) {
            output.push(char);
        }

        lastChar[0] = char;
    }

    return output.join('');
}

console.log(minify(input));
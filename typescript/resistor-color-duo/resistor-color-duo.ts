let Colors = [
    "black",
    "brown",
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "violet",
    "grey",
    "white",
];

export function decodedValue(colors: Array<string>): number {
    let sum = "";
    colors.slice(0, 2).forEach((element) => {
        sum += Colors.indexOf(element);
    });

    return parseInt(sum);
}

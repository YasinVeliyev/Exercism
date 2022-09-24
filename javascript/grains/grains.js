/**
 * You can use the bigint type and BigInt global object to support numbers below
 * Number.MIN_SAFE_INTEGER and above NUMBER.MAX_SAFE_INTEGER.
 *
 * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt
 */

//
// This is only a SKELETON file for the 'Grains' exercise. It's been provided as a
// convenience to get you started writing code faster.
//
export const square = (sq) => {
    if (sq > 64 || sq < 1) {
        throw "square must be between 1 and 64";
    }
    return BigInt(Math.pow(2, sq - 1));
};

export const total = () => {
    return BigInt(Math.pow(2, 64)) - BigInt(1);
};

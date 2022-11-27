//
// This is only a SKELETON file for the 'Change' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

// class Change {
//     calculate(coinArray, target) {
//         if (target < 0) {
//             throw new Error("Negative totals are not allowed.");
//         }

//         let t = target;
//         let arr = [];

//         let newCoinArray = [...coinArray];
//         newCoinArray.reverse();
//         let start = 0;
//         let index = 0;
//         while (start < newCoinArray.length) {
//             let i = newCoinArray[index];
//             console.log(arr, t, i, start, index);
//             if (parseInt(t / i)) {
//                 arr = arr.concat(new Array(parseInt(t / i)).fill(i));
//             } else if (t == 0) {
//                 arr.reverse();

//                 return arr;
//             }
//             start++;
//             index++;
//             if (t < i && start <= newCoinArray.length) {
//                 if (arr.length) {
//                     t += arr.pop();
//                 }

//                 start--;
//                 continue;
//             }
//             t %= i;
//             if (t && start == 0) {
//                 break;
//             }
//         }

//         if (t) {
//             console.log("Aaa");
//             start = 0;
//             arr = [];
//             t = target;
//             first: while (start < coinArray.length) {
//                 let limit = this.ekob(...coinArray.slice(start, start + 2));
//                 for (let i = 0; i < limit; i += coinArray[start]) {
//                     if (t % coinArray[start + 1] == 0) {
//                         if (start + 1 >= coinArray.length - 1) {
//                             arr = arr.concat(new Array(t / coinArray[start + 1]).fill(coinArray[start + 1]));
//                             t = t % coinArray[start + 1];
//                         }
//                         break;
//                     } else if (t == 0) {
//                         break first;
//                     }
//                     t -= coinArray[start];
//                     arr.push(coinArray[start]);
//                 }

//                 start += 1;
//             }
//             if (t) {
//                 throw new Error(`The total ${target} cannot be represented in the given currency.`);
//             }
//             return arr;
//         }
//     }

//     ebob(a, b) {
//         while (b > 0) {
//             [b, a] = [a % b, b];
//         }
//         return a;
//     }

//     ekob(a, b) {
//         return (a * b) / this.ebob(a, b);
//     }
// }
class Change {
    calculate(coinArray, target) {
        if (target < 0) {
            throw "Negative totals are not allowed.";
        }
        const change = [[]];
        for (let i = 0; i < target; ++i) {
            if (!change[i]) continue;
            coinArray.forEach((c) => {
                const cs = [...change[i], c];

                if (!change[c + i] || cs.length < change[c + i].length) {
                    change[c + i] = cs;
                }
            });
        }
        if (!change[target]) {
            throw `The total ${target} cannot be represented in the given currency.`;
        }

        return change[target];
    }
}

const change = new Change();
// // const result = change.calculate([2, 5, 10, 20, 50], 21);
const result1 = change.calculate([1, 5, 10, 21, 25], 63);
// const result2 = change.calculate([1, 2, 5, 10, 20, 50, 100], 999);
// // // const result3 = change.calculate([5, 10], 3);
// // // const result4 = change.calculate([5, 10], 94);
// // const result5 = change.calculate([5, 10], 0);
// const result6 = change.calculate([20, 50, 100, 200], 960);
// // console.log(result);
console.log(result1);
// console.log(
//     result2.reduce((a, b) => a + b),
//     result2
// );
// // console.log(result3);
// // console.log(result4);
// // console.log(result5);
// console.log(result6);
// console.log(change.ekob(5, 2));

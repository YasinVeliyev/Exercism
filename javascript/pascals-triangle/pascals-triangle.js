//
// This is only a SKELETON file for the 'Pascals Triangle' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

function factorial(num) {
	if (num > 1) {
		return num * factorial(num - 1);
	}
	return 1;
}

export const rows = (degree) => {
	let rows = [];
	for (let i = 0; i < degree; i++) {
		let row = [];
		for (let j = 0; j <= i; j++) {
			row.push(factorial(i) / factorial(j) / factorial(i - j));
		}
		rows.push(row);
	}
	return rows;
};

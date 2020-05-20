fn main() {
    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let _tup = (500, 6.4, 1);

    let (_x, y, _z) = _tup;

    println!("The value of y is: {}", y);

    let _x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = _x.0;

    let _six_point_four = _x.1;

    let _one = _x.2;

    let _a = [1, 2, 3, 4, 5]; // array type

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let _a: [i32; 5] = [1, 2, 3, 4, 5]; //You would write an array’s type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array, like so:

    let _b = [3; 5]; // = let a = [3, 3, 3, 3, 3];

    let _c = [1, 2, 3, 4, 5];

    let _first = _a[0];
    let _second = _a[1];
}

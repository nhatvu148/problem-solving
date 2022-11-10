let myObj = { a: 10, b: 12 };
// myObj = null;
let myFunc = () => {
  return 100;
};
// myFunc = undefined;

console.log(myObj?.a);
delete myObj?.a;
console.log(myObj?.["a"]);
console.log(myFunc?.());

let a = 123;
// a = 0;
a = null;
let b = 456;
console.log(a ?? b);
console.log(a || b);

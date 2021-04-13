function stripProperty(obj, prop) {
  // write your code here
  let _prop, remaining;
  if (Object.keys(obj).includes(prop)) {
      ({[prop]: _prop, ...remaining} = obj);
  } else {
      remaining = {...obj};
  }
  return remaining;
}

const x = {
    bar: "hello",
    baz: "there",
    foo: 1123
}

const Axios = require("axios");

async function getCountryName(code) {
    // write your code here
    let countries;
    let resCountry;
    let page = 1;
    while (!resCountry) {
        const res = await Axios.get(`https://jsonmock.hackerrank.com/api/countries?page=${page++}`);
        countries = res.data.data;
        resCountry = countries.find(c => c.alpha2Code === code);
        console.log(resCountry?.name);
    }
}
getCountryName("AF")

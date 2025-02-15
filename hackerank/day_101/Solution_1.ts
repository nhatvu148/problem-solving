const promise = new Promise((resolve, reject) => {
  setTimeout(() => {
    reject("Promise Rejected from TimeOut");
  }, 0);
  resolve("Promise Resolved");
  setTimeout(() => {
    resolve("Promise Resolved from TimeOut");
  }, 0);
  console.log("End of Promise");
});

promise.then((data) => console.log(data)).catch((err) => console.log(err));

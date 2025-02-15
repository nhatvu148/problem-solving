async function avgRotorSpeed(statusQuery, parentId) {
  try {
    let allDevices = [];
    let currentPage = 1;
    let totalPages = 1;

    while (currentPage <= totalPages) {
      const response = await fetch(
        `https://jsonmock.hackerrank.com/api/iot_devices/search?status=${statusQuery}&page=${currentPage}`
      );
      const data = await response.json();

      if (currentPage === 1) {
        totalPages = data.total_pages;
      }

      allDevices = allDevices.concat(data.data);

      currentPage++;
    }

    // Filter devices that match the parentId
    const matchingDevices = allDevices.filter(
      (device) => device.parent && device.parent.id === parentId
    );

    // If no devices match, return 0
    if (matchingDevices.length === 0) {
      return 0;
    }

    // Calculate the average rotor speed
    const totalRotorSpeed = matchingDevices.reduce(
      (sum, device) => sum + device.operatingParams.rotorSpeed,
      0
    );

    const averageRotorSpeed = totalRotorSpeed / matchingDevices.length;

    return Math.floor(averageRotorSpeed);
  } catch (error) {
    console.error("Error fetching data:", error);
    return 0; // Return 0 in case of error as a fallback
  }
}

async function main() {
  const result = await avgRotorSpeed('STOP', 2);
//   const result = await avgRotorSpeed("RUNNING", 7);
  console.log(result);
}

// Execute main function
main().catch((error) => console.error(error));

const hre = require("hardhat");

async function main() {
  // Get the current timestamp
  const currentTimestampInSeconds = Math.round(Date.now() / 1000);
  // Add one year in seconds (365 * 24 * 60 * 60)
  const unlockTime = currentTimestampInSeconds + 365 * 24 * 60 * 60;

  // Get contract factory
  const MyContract = await hre.ethers.getContractFactory("MyContract");

  // Deploy contract with 1 ETH locked
  const myContract = await MyContract.deploy(unlockTime, {
    value: hre.ethers.parseEther("1"),
  });

  await myContract.deployed();

  console.log(`Lock with 1 ETH deployed to ${lock.address}`);
}

// Handle errors
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

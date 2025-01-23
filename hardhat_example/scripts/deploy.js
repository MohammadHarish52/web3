const hre = require("hardhat");

async function main() {
  // Get contract factory
  const MyContract = await hre.ethers.getContractFactory("MyContract");

  // Deploy without arguments since constructor is empty
  const myContract = await MyContract.deploy();

  await myContract.waitForDeployment();

  const address = await myContract.getAddress();

  console.log(`MyContract deployed to: ${address}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

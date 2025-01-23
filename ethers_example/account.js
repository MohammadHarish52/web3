import { ethers } from "ethers";
const rpcUrl = "https://cloudflare-eth.com";

const provider = new ethers.JsonRpcProvider(rpcUrl);

const address = "0x6B175474E89094C44Da98b954EedeAC495271d0F";

const ERC20_ABI = [
  "function name() view returns (string)",
  "function symbol() view returns (string)",
  "function totalSupply() view returns (uint256)", // Fixed typo
  "function balanceOf(address owner) view returns (uint256)",
  "function transfer(address to, uint256 amount)",
  "event Transfer(address indexed from, address indexed to, uint256 amount)",
];
const contract = new ethers.Contract(address, ERC20_ABI, provider);
async function main() {
  try {
    const name = await contract.name();
    const symbol = await contract.symbol();
    const totalSupply = await contract.totalSupply();
    const balance = await contract.balanceOf(address);
    console.log(`Name: ${name}`);
    console.log(`Symbol: ${symbol}`);
    console.log(`Total Supply: ${totalSupply}`);
    console.log(`Balance: ${balance}`);
  } catch (error) {
    console.error("Error interacting with contract:", error);
  }
}

main().catch(console.error);

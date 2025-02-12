const address = pg.wallet.publicKey;
const accountInfo = await pg.connection.getAccountInfo(address);

// Print the account information
console.log(JSON.stringify(accountInfo, null, 2));

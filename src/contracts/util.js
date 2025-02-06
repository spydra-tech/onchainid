const { Keypair } = require("@stellar/stellar-sdk");
const { Server } = require("@stellar/stellar-sdk/rpc");
const { rpcUrl } = require("./config.js");

const generateFundedKeypair = async () => {
    const keypair = Keypair.random();
    const server = new Server(rpcUrl);
    await server.requestAirdrop(keypair.publicKey());
    return keypair
};

module.exports = {
    generateFundedKeypair
};
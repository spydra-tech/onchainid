require('dotenv').config();
const { Networks } = require("@stellar/stellar-sdk");

const rpcUrl = process.env.PUBLIC_SOROBAN_RPC_URL ?? "https://soroban-testnet.stellar.org";
const networkPassphrase = Networks.TESTNET;
const wasmHash = process.env.PUBLIC_SOROBAN_WASM_HASH;

module.exports = {
    rpcUrl,
    networkPassphrase,
    wasmHash
};

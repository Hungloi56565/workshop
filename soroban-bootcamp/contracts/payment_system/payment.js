import * as StellarSdk from "@stellar/stellar-sdk";
import { isConnected, getAddress, signTransaction } from "@stellar/freighter-api";


const CONTRACT_ID = "CDH5AOFW6LNHJYR5QHZAIHKR4O3RE6HLZBPXU2VFSIHR7E6MLO6IORUE";
const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = "Test SDF Network ; September 2015";

// Secret key của tài khoản testnet (lấy từ faucet)
const secret = "SCNZL5D256XOUXX7SS67C2GQXNVLLK3IXKO2BU2VY7XS77HIAIEUHVE4"; 
const keypair = StellarSdk.Keypair.fromSecret(secret);

async function callContract(funcName, ...args) {
  const server = new StellarSdk.SorobanRpc.Server(RPC_URL);
  const account = await server.getAccount(keypair.publicKey());
  const contract = new StellarSdk.Contract(CONTRACT_ID);

  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: StellarSdk.BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call(funcName, ...args))
    .setTimeout(30)
    .build();

  tx.sign(keypair); // ký bằng secret key

  const result = await server.sendTransaction(tx);
  console.log("Kết quả:", result);
}

(async () => {
  await callContract("initialize");
})();

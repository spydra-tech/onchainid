const {
    Keypair,
    Contract,
    SorobanRpc,
    TransactionBuilder,
    Networks,
    BASE_FEE,
    nativeToScVal,
    Address,
    sign,
  } = require("@stellar/stellar-sdk");
  const { Client, basicNodeSigner } = require("@stellar/stellar-sdk/contract");
  const { Server } = require("@stellar/stellar-sdk/rpc");
  const { rpcUrl, networkPassphrase, wasmHash } = require("./contracts/config.js");
  const { generateFundedKeypair } = require("./contracts/util.js");
  const { ClaimTopic, KeyPurpose, keyType } = require("./contracts/constants.js");

  (async () => {
    console.log('Generating User key pair');
    const userKeypair = await generateFundedKeypair();      // User's key which is used to deploy the identity contract
    console.log(`User public key: ${userKeypair.publicKey()}`);

    console.log('Generating Issuer key pair');
    const issuerKeypair = await generateFundedKeypair();    // The key of the issuer which will be used to sign claims
    console.log(`Issuer public key: ${issuerKeypair.publicKey()}`);

    const { signTransaction } = basicNodeSigner(userKeypair, networkPassphrase);
    console.log('\nDeploying Identity Contract');
    //Deploy the identity contract
    const deployTx = await Client.deploy(
        null,
        {
          networkPassphrase: networkPassphrase,
          rpcUrl,
          wasmHash,
          publicKey: userKeypair.publicKey(),
          signTransaction,
        }
      );
    const { result: client } = await deployTx.signAndSend();
    const identityContractId = client.options.contractId;
    const identityAddress = new Address(identityContractId)
    console.log(`Identity Contract Address: ${identityContractId}`);

    //Add issuer key with purpose as claim signer - purpose 2. Key type 1 is ECDSA
    console.log('\nAdding claim signer key');
    const addKeyTx = await client.add_key({
        key: issuerKeypair.rawPublicKey(),
        purpose: KeyPurpose.CLAIM_SIGNER,
        key_type: keyType.ECDSA,
      });
    const { result } = await addKeyTx.signAndSend()
    console.log(`Result of adding key=${JSON.stringify(result)}`);

    //Add a claim signed by the issuer.
    //In a real world scenario, this will be done by an issuer who has validated the identity of a user.
    //We will be adding a claim that kyc check has been performed and it has passed.
    const claimTopic = ClaimTopic.SPECIFIC_KYC_STATUS;
    const claimTopicBuffer = Buffer.allocUnsafe(4);
    claimTopicBuffer.writeUInt32BE(claimTopic); 
    const data = "true";

    //Calculate the signature of concatenated identity contract address, claim topic and data
    let sigData = identityAddress.toBuffer();
    sigData = Buffer.concat([sigData, claimTopicBuffer]);
    sigData = Buffer.concat([sigData, Buffer.from(data)]);
    const sig = issuerKeypair.sign(sigData);

    //Add claim
    console.log('\nAdding KYC claim for user');
    const addClaimTx = await client.add_claim({
        topic: claimTopic,
        scheme: 1,
        issuer: issuerKeypair.rawPublicKey(),
        signature: sig,
        data: Buffer.from(data),
        uri: '',
    });
    const addClaimResult = await addClaimTx.signAndSend()
    console.log(`Claim id=${JSON.stringify(addClaimResult.result)}`);
    const claimId = addClaimResult.result;

    //Retreive claims of user from the identity contract
    console.log('\nRetrieve user claims from contract');
    const getClaimTx = await client.get_claim({
        claim_id: claimId,
      });

    const [gotTopic, gotScheme, gotIssuer, gotSig, gotData, gotUri] = getClaimTx.result.value;
    console.log(`Retrieved claims:`);
    console.log(`Topic=${gotTopic}, Scheme=${gotScheme}, Issuer=${Address.account(gotIssuer).toString()}, Signature=${gotSig}, Data=${gotData.toString()}, Uri=${gotUri}`);

    //Validate the claim to ensure that it has been issued by the expected issuer
    console.log('\nValidating claim');
    //validate claim
    const validateClaimTx = await client.is_claim_valid({
        identity: identityAddress.toBuffer(),
        issuer: issuerKeypair.rawPublicKey(),
        topic: claimTopic,
        sig: sig,
        data: Buffer.from(data),
      });
      console.log(`Validation result=${JSON.stringify(validateClaimTx.result)}`);
  })();
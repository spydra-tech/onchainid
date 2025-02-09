const { Address } = require("@stellar/stellar-sdk");
  const { Client, basicNodeSigner } = require("@stellar/stellar-sdk/contract");
  const { rpcUrl, networkPassphrase, wasmHash } = require("./contracts/config.js");
  const { generateFundedKeypair } = require("./contracts/util.js");
  const { ClaimTopic, KeyPurpose, keyType } = require("./contracts/constants.js");

  (async () => {
    console.log("Generating User Alice's key pair");
    const aliceKeypair = await generateFundedKeypair();      // User's key which is used to deploy the identity contract
    console.log(`User public key: ${aliceKeypair.publicKey()}`);

    console.log('Generating Issuer key pair');
    const issuerKeypair = await generateFundedKeypair();    // The key of the issuer which will be used to sign claims
    console.log(`Issuer public key: ${issuerKeypair.publicKey()}`);

    


    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // The below section demonstrates the deployment of an identity contract by a user Alice.
    // Alice also adds a claim signer key to the contract which authorizes the issuer to sign claims about Alice

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    console.log('\n///////////////////////////////////////////////////////////////');
    console.log('Excecution on behalf of Alice');
    console.log('///////////////////////////////////////////////////////////////');
    console.log('\nDeploying Identity Contract');

    const { signTransaction } = basicNodeSigner(aliceKeypair, networkPassphrase);
    //Deploy the identity contract
    const deployTx = await Client.deploy(
        null,
        {
          networkPassphrase: networkPassphrase,
          rpcUrl,
          wasmHash,
          publicKey: aliceKeypair.publicKey(),
          signTransaction,
        }
      );
    const { result: client } = await deployTx.signAndSend();
    const identityContractId = client.options.contractId;
    const identityAddress = new Address(identityContractId)
    console.log(`Identity Contract Address: ${identityContractId}`);

    //Add issuer key with purpose as claim signer - purpose 3. Key type 1 is ECDSA
    console.log('\nAdding claim signer key');
    const addKeyTx = await client.add_key({
        key: issuerKeypair.rawPublicKey(),
        purpose: KeyPurpose.CLAIM_SIGNER,
        key_type: keyType.ECDSA,
      });
    const { result } = await addKeyTx.signAndSend()
    console.log(`Result of adding key=${JSON.stringify(result)}`);




    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // The below section demonstrates the issuer adding a claim to the identity contract of Alice.
    // The issuer signs the claim using the key added by Alice in the previous step which authorized the issuer to do so.
    // Typically, the issuer would have their own app which would have validated the identity of Alice before adding the claim.

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////  

    console.log('\n///////////////////////////////////////////////////////////////');
    console.log('Excecution on behalf of Issuer');
    console.log('///////////////////////////////////////////////////////////////');
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
    console.log(`Kyc Claim id=${JSON.stringify(addClaimResult.result)}`);
    const claimId = addClaimResult.result;

    //We will be adding a claim that kyc check has been performed and it has passed.
    const claimTopicName = ClaimTopic.FIRST_NAME_CLEAR;
    const claimTopicNameBuffer = Buffer.allocUnsafe(4);
    claimTopicNameBuffer.writeUInt32BE(claimTopicName); 
    const nameData = "Alice";

    //Calculate the signature of concatenated identity contract address, claim topic and data
    let sigNameData = identityAddress.toBuffer();
    sigNameData = Buffer.concat([sigNameData, claimTopicNameBuffer]);
    sigNameData = Buffer.concat([sigNameData, Buffer.from(nameData)]);
    const nameSig = issuerKeypair.sign(sigNameData);

    //Add claim
    console.log('\nAdding Name claim for user');
    const addNameClaimTx = await client.add_claim({
        topic: claimTopicName,
        scheme: 1,
        issuer: issuerKeypair.rawPublicKey(),
        signature: nameSig,
        data: Buffer.from(nameData),
        uri: '',
    });
    const addNameClaimResult = await addNameClaimTx.signAndSend()
    console.log(`Name Claim id=${JSON.stringify(addNameClaimResult.result)}`);
    const nameClaimId = addNameClaimResult.result;



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // The below section demonstrates a verifier who wants to use Alice's identity information retreiving and
    // verifying the claims from Alice's identity contract.
    // Typically, this is done by an Asset owner who wants to ensure that the user has passed KYC before allowing them to trade.
    // This check can also be done from inside a token smart contract before transferring tokens to a recipient.

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    console.log('\n///////////////////////////////////////////////////////////////');
    console.log('Excecution on behalf of Verifier');
    console.log('///////////////////////////////////////////////////////////////');

    //Retreive Kyc claim of user from the identity contract
    console.log('\nRetrieve user Kyc claim from contract');
    const getClaimTx = await client.get_claim({
        claim_id: claimId,
      });

    const [gotTopic, gotScheme, gotIssuer, gotSig, gotData, gotUri] = getClaimTx.result.value;
    console.log(`Retrieved Kyc Claim:`);
    console.log(`Topic=${gotTopic}, Scheme=${gotScheme}, Issuer=${Address.account(gotIssuer).toString()}, Signature=${gotSig}, Data=${gotData.toString()}, Uri=${gotUri}`);

    //Retreive Name claim of user from the identity contract
    console.log('\nRetrieve user Name claim from contract');
    const getNameClaimTx = await client.get_claim({
        claim_id: nameClaimId,
      });

    const [gotNameTopic, gotNameScheme, gotNameIssuer, gotNameSig, gotNameData, gotNameUri] = getNameClaimTx.result.value;
    console.log(`Retrieved Name Claim:`);
    console.log(`Topic=${gotNameTopic}, Scheme=${gotNameScheme}, Issuer=${Address.account(gotNameIssuer).toString()}, Signature=${gotNameSig}, Data=${gotNameData.toString()}, Uri=${gotNameUri}`);

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
// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, Config, Document, KeyType, Network } from '@iota/identity-wasm';

/**
    This example shows how a DID document can be created on a private tangle.
    It can be run together with a local hornet node.
    Refer to https://github.com/iotaledger/one-click-tangle/tree/chrysalis/hornet-private-net
    for setup instructions.
**/
async function createIdentityPrivateTangle(restURL, networkName) {
    // This name needs to match the id of the network or part of it.
    // Since the id of the one-click private tangle is `private-tangle`
    // but we can only use 6 characters, we use just `tangle`.
    const network = Network.try_from_name(networkName == "" ? "tangle" : networkName);

    // Create a DID Document (an identity).
    const { doc, key } = new Document(KeyType.Ed25519, network.toString());

    // Sign the DID Document with the generated key.
    doc.sign(key);

    // Create a client configuration and set the custom network.
    const config = new Config();
    config.setNetwork(network);

    // This URL points to the REST API of the locally running hornet node.
    config.setNode(restURL == "" ? "http://127.0.0.1:14265/" : restURL);

    // Create a client instance from the configuration to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const receipt = await client.publishDocument(doc.toJSON());

    // Make sure the DID can be resolved on the private tangle
    const resolved = await client.resolve(doc.id.toString());

    console.log(`Published the DID document to the private tangle:`);
    console.log(resolved);

    // Return the results.
    return { key, resolved, receipt };
}

export {createIdentityPrivateTangle};
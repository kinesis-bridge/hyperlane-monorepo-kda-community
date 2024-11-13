import { Verifier } from './dto/in/build-pact-tx.dto';
import {
  poll,
  IPollRequestBody,
  IPollResponse,
  send,
  localRaw,
  IRequestKeys,
  ISendRequestBody,
  LocalRequestBody,
  ICommandResult,
} from '@kadena/chainweb-node-client';
import chainweb from '@kadena/chainwebjs';
import {
  IBlockHeader,
  IBlockPayloads,
  ITransactionElement,
  IEventData,
} from '@kadena/chainwebjs/lib/types';
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import {
  ChainId,
  ICommand,
  IUnsignedCommand,
  Pact,
  createClient,
} from '@kadena/client';
import { Injectable } from '@nestjs/common';

@Injectable()
export class AppService {
  async getBlocks(
    host: string,
    network: string,
    chainId: number | string,
    from: number,
    to: number,
  ): Promise<IBlockPayloads<ITransactionElement>[]> {
    return chainweb.block.range(chainId, from, to, network, host);
  }

  async getBlockByHash(
    host: string,
    network: string,
    chainId: number | string,
    hash: string,
  ): Promise<IBlockPayloads<ITransactionElement>> {
    return chainweb.block.blockHash(chainId, hash, network, host);
  }

  async getBlockByHeight(
    host: string,
    network: string,
    chainId: number | string,
    height: number,
  ): Promise<IBlockPayloads<ITransactionElement>> {
    return chainweb.block.height(chainId, height, network, host);
  }

  async getEvents(
    host: string,
    network: string,
    chainId: number | string,
    from: number,
    to: number,
  ): Promise<IEventData[]> {
    return chainweb.event.range(chainId, from, to, network, host);
  }

  async getEventByHash(
    host: string,
    network: string,
    chainId: number | string,
    hash: string,
  ): Promise<IEventData[]> {
    return chainweb.event.blockHash(chainId, hash, network, host);
  }

  async getEventByHeight(
    host: string,
    network: string,
    chainId: number | string,
    height: number,
  ): Promise<IEventData[]> {
    return chainweb.event.height(chainId, height, network, host);
  }

  async getHeaders(
    host: string,
    network: string,
    chainId: number | string,
    from: number,
    to: number,
  ): Promise<IBlockHeader[]> {
    return chainweb.header.range(chainId, from, to, network, host);
  }

  async getHeaderByHash(
    host: string,
    network: string,
    chainId: number | string,
    hash: string,
  ): Promise<IBlockHeader> {
    return chainweb.header.blockHash(chainId, hash, network, host);
  }

  async getHeaderByHeight(
    host: string,
    network: string,
    chainId: number | string,
    height: number,
  ): Promise<IBlockHeader> {
    return chainweb.header.height(chainId, height, network, host);
  }

  async getHeightWithDepth(
    host: string,
    network: string,
    depth: number,
  ): Promise<number> {
    const cut = await chainweb.cut.current(network, host);
    return cut.hashes['0'].height - depth;
  }

  async getTxs(
    host: string,
    network: string,
    chainId: number | string,
    from: number,
    to: number,
  ): Promise<ITransactionElement[]> {
    return chainweb.transaction.range(chainId, from, to, network, host);
  }

  async getTxByHash(
    host: string,
    network: string,
    chainId: number | string,
    hash: string,
  ): Promise<ITransactionElement[]> {
    return chainweb.transaction.blockHash(chainId, hash, network, host);
  }

  async getTxByHeight(
    host: string,
    network: string,
    chainId: number | string,
    height: number,
  ): Promise<ITransactionElement[]> {
    return chainweb.transaction.height(chainId, height, network, host);
  }

  async buildPactTx(
    host: string,
    network: string,
    chainId: number | string,
    pactCode: string,
    signer: string,
    senderAccount: string,
    gasLimit: number,
    verifiers: Verifier[],
  ): Promise<IUnsignedCommand> {
    const creationTime = () => Math.round(new Date().getTime() / 1000);

    let builder = Pact.builder
      .execution(pactCode)
      .setMeta({
        chainId: chainId.toString() as ChainId,
        senderAccount: senderAccount,
        gasLimit: gasLimit,
        creationTime: creationTime() - 28800,
        ttl: 30000,
      })
      .setNetworkId(network);
    if (signer.trim().length != 0) {
      builder = builder.addSigner(signer);
    }

    verifiers.forEach((verifier) => {
      builder.addVerifier(
        {
          name: verifier.name,
          proof: verifier.proof,
        },
        (withCapability) => {
          return verifier.capabilities.map((cap) => {
            const [capName, ...args] = cap;
            const parsedArgs = args.map((arg) => {
              if (typeof arg === 'string') {
                try {
                  // Try to parse arg as JSON
                  return JSON.parse(arg);
                } catch (e) {
                  // If parsing fails, return the original string
                  return arg;
                }
              } else {
                // If arg is not a string, return it as is
                return arg;
              }
            });
            return withCapability(capName, ...parsedArgs);
          });
        },
      );
    });

    return builder.createTransaction();
  }

  async poll(
    requestBody: IPollRequestBody,
    apiHost: string,
  ): Promise<IPollResponse> {
    return poll(requestBody, apiHost);
  }

  async send(
    requestBody: ISendRequestBody,
    apiHost: string,
  ): Promise<IRequestKeys> {
    return send(requestBody, apiHost);
  }

  async continueTransferRemote(
    host: string,
    network: string,
    chain: number | string,
    pactId: string,
    destinationChainId: ChainId,
    step: number,
    rollback: boolean,
  ): Promise<ICommandResult> {
    const pollOptions = {
      timeout: 60_000, // 60 seconds
      interval: 5_000, // 5 seconds
    };
    const gasStationPayer = 'kadena-xchain-gas';
    const gasStationGasLimit = 850;

    const client = createClient(
      ({ chainId, networkId }: { chainId: ChainId; networkId: string }) =>
        `${host}chainweb/0.0/${networkId}/chain/${chainId}/pact`,
    );

    // 1. Create SPV proof
    const proof = await client.pollCreateSpv(
      {
        requestKey: pactId,
        networkId: network,
        chainId: chain.toString() as ChainId,
      },
      destinationChainId,
      pollOptions,
    );

    // 2. Build the continuation transaction
    const builder = Pact.builder
      .continuation({
        pactId,
        proof,
        rollback,
        step,
      })
      .setNetworkId(network)
      .setMeta({
        chainId: destinationChainId,
        senderAccount: gasStationPayer,
        gasLimit: gasStationGasLimit,
      });
    const tx = builder.createTransaction();

    // 3. Submit the transaction
    const finishTransactionDescriptor = await client.submit(tx as ICommand);

    // 4. Poll the status of the transaction
    const result = await client.pollStatus(
      finishTransactionDescriptor,
      pollOptions,
    );
    return result[finishTransactionDescriptor.requestKey];
  }

  async local(
    requestBody: LocalRequestBody,
    apiHost: string,
    preflight: boolean,
    signatureVerification: boolean,
  ): Promise<ICommandResult> {
    const rsp = await localRaw(requestBody, apiHost, {
      signatureVerification: signatureVerification,
      preflight: preflight,
    });
    if ('preflightResult' in rsp) {
      return rsp.preflightResult;
    }
    return rsp;
  }
}

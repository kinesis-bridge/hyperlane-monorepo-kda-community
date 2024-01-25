import { Injectable } from '@nestjs/common';
import chainweb from '@kadena/chainwebjs';
import {
  IBlockHeader,
  IBlockPayloads,
  ITransactionElement,
  IEventData,
} from '@kadena/chainwebjs/lib/types';
import { ChainId, IUnsignedCommand, Pact } from '@kadena/client';
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

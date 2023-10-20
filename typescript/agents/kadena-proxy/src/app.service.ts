import { Injectable } from '@nestjs/common';
import chainweb from '@kadena/chainwebjs';
import {
  IBlockHeader,
  IBlockPayloads,
  ITransactionElement,
  IEventData,
} from '@kadena/chainwebjs/lib/types';
import { ChainId, IUnsignedCommand, Pact } from '@kadena/client';

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
  ): Promise<IUnsignedCommand> {
    const builder = Pact.builder
      .execution(pactCode)
      .addSigner(signer)
      .setMeta({
        chainId: chainId as ChainId,
        senderAccount: senderAccount,
      })
      .setNetworkId(network)
      .createTransaction();
    return builder;
  }
}

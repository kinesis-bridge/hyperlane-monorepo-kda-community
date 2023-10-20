import { Body, Controller, Get, Post, Query } from '@nestjs/common';
import { ApiResponse } from '@nestjs/swagger';
import { AppService } from './app.service';
import {
  IBlockHeader,
  IBlockPayloads,
  IEventData,
  ITransactionElement,
} from '@kadena/chainwebjs/lib/types';
import { IUnsignedCommand } from '@kadena/client';
import { BuildPactTxDto } from './dto';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Get('blocks')
  async getBlocks(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('from') from: number,
    @Query('to') to: number,
  ): Promise<IBlockPayloads<ITransactionElement>[]> {
    return this.appService.getBlocks(host, network, chainId, from, to);
  }

  @Get('block_by_hash')
  async getBlockByHash(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('hash') hash: string,
  ): Promise<IBlockPayloads<ITransactionElement>> {
    return this.appService.getBlockByHash(host, network, chainId, hash);
  }

  @Get('block_by_height')
  async getBlockByHeight(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('height') height: number,
  ): Promise<IBlockPayloads<ITransactionElement>> {
    return this.appService.getBlockByHeight(host, network, chainId, height);
  }
  @ApiResponse({
    status: 200,
  })
  @Post('build_pact_tx')
  async buildTx(
    @Body() buildPactTxDto: BuildPactTxDto,
  ): Promise<IUnsignedCommand> {
    return this.appService.buildPactTx(
      buildPactTxDto.host,
      buildPactTxDto.network,
      buildPactTxDto.chainId,
      buildPactTxDto.pactCode,
      buildPactTxDto.signer,
      buildPactTxDto.senderAccount,
    );
  }

  @Get('events')
  async getEvents(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('from') from: number,
    @Query('to') to: number,
  ): Promise<IEventData[]> {
    return this.appService.getEvents(host, network, chainId, from, to);
  }

  @Get('event_by_hash')
  async getEventByHash(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('hash') hash: string,
  ): Promise<IEventData[]> {
    return this.appService.getEventByHash(host, network, chainId, hash);
  }

  @Get('event_by_height')
  async getEventByHeight(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('height') height: number,
  ): Promise<IEventData[]> {
    return this.appService.getEventByHeight(host, network, chainId, height);
  }

  @Get('headers')
  async getHeaders(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('from') from: number,
    @Query('to') to: number,
  ): Promise<IBlockHeader[]> {
    return this.appService.getHeaders(host, network, chainId, from, to);
  }

  @Get('header_by_hash')
  async getHeaderByHash(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('hash') hash: string,
  ): Promise<IBlockHeader> {
    return this.appService.getHeaderByHash(host, network, chainId, hash);
  }

  @Get('header_by_height')
  async getHeaderByHeight(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('height') height: number,
  ): Promise<IBlockHeader> {
    return this.appService.getHeaderByHeight(host, network, chainId, height);
  }

  @Get('txs')
  async getTxs(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: string | number,
    @Query('from') from: number,
    @Query('to') to: number,
  ): Promise<ITransactionElement[]> {
    return this.appService.getTxs(host, network, chainId, from, to);
  }

  @Get('tx_by_hash')
  async getTxByHash(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('hash') hash: string,
  ): Promise<ITransactionElement[]> {
    return this.appService.getTxByHash(host, network, chainId, hash);
  }

  @Get('tx_by_height')
  async getTxByHeight(
    @Query('host') host: string,
    @Query('network') network: string,
    @Query('chain_id') chainId: number,
    @Query('height') height: number,
  ): Promise<ITransactionElement[]> {
    return this.appService.getTxByHeight(host, network, chainId, height);
  }
}

import { Body, Controller, Get, HttpCode, Post, Query } from '@nestjs/common';
import { ApiResponse } from '@nestjs/swagger';
import { AppService } from './app.service';
import { BlockHeaderDto } from './dto/out/block-header.dto';
import { BlockPayloadsDto } from './dto/out/block-payloads.dto';
import { BuildPactTxDto } from './dto/in/build-pact-tx.dto';
import { EventDataDto } from './dto/out/event-data.dto';
import { TransactionElementDto } from './dto/out/transaction-element.dto';
import { GetItemByHeightDto } from './dto/in/get-item-by-height.dto';
import { GetItemsDto } from './dto/in/get-items.dto';
import { GetItemByHashDto } from './dto/in/get-item-by-hash.dto';
import { GetHeightDto } from './dto/in/get-height.dto';
import { UnsignedCommandDto } from './dto/out/unsigned-command.dto';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @ApiResponse({
    isArray: true,
    type: BlockPayloadsDto,
  })
  @Get('blocks')
  async getBlocks(
    @Query() queryParams: GetItemsDto,
  ): Promise<BlockPayloadsDto[]> {
    return this.appService.getBlocks(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.from,
      queryParams.to,
    );
  }

  @ApiResponse({
    type: BlockPayloadsDto,
  })
  @Get('block_by_hash')
  async getBlockByHash(
    @Query() queryParams: GetItemByHashDto,
  ): Promise<BlockPayloadsDto> {
    return this.appService.getBlockByHash(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.hash,
    );
  }

  @ApiResponse({
    type: BlockPayloadsDto,
  })
  @Get('block_by_height')
  async getBlockByHeight(
    @Query() queryParams: GetItemByHeightDto,
  ): Promise<BlockPayloadsDto> {
    return this.appService.getBlockByHeight(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.height,
    );
  }

  @ApiResponse({
    status: 200,
    type: UnsignedCommandDto,
  })
  @Post('build_pact_tx')
  @HttpCode(200)
  async buildTx(
    @Body() buildPactTxDto: BuildPactTxDto,
  ): Promise<UnsignedCommandDto> {
    return this.appService.buildPactTx(
      buildPactTxDto.host,
      buildPactTxDto.network,
      buildPactTxDto.chain_id,
      buildPactTxDto.pactCode,
      buildPactTxDto.signer,
      buildPactTxDto.senderAccount,
    );
  }

  @ApiResponse({
    isArray: true,
    type: EventDataDto,
  })
  @Get('events')
  async getEvents(@Query() queryParams: GetItemsDto): Promise<EventDataDto[]> {
    return this.appService.getEvents(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.from,
      queryParams.to,
    );
  }

  @ApiResponse({
    isArray: true,
    type: EventDataDto,
  })
  @Get('event_by_hash')
  async getEventByHash(
    @Query() queryParams: GetItemByHashDto,
  ): Promise<EventDataDto[]> {
    return this.appService.getEventByHash(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.hash,
    );
  }

  @ApiResponse({
    isArray: true,
    type: EventDataDto,
  })
  @Get('event_by_height')
  async getEventByHeight(
    @Query() queryParams: GetItemByHeightDto,
  ): Promise<EventDataDto[]> {
    return this.appService.getEventByHeight(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.height,
    );
  }

  @ApiResponse({
    isArray: true,
    type: BlockHeaderDto,
  })
  @Get('headers')
  async getHeaders(
    @Query() queryParams: GetItemsDto,
  ): Promise<BlockHeaderDto[]> {
    return this.appService.getHeaders(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.from,
      queryParams.to,
    );
  }

  @ApiResponse({
    type: BlockHeaderDto,
  })
  @Get('header_by_hash')
  async getHeaderByHash(
    @Query() queryParams: GetItemByHashDto,
  ): Promise<BlockHeaderDto> {
    return this.appService.getHeaderByHash(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.hash,
    );
  }

  @ApiResponse({
    type: BlockHeaderDto,
  })
  @Get('header_by_height')
  async getHeaderByHeight(
    @Query() queryParams: GetItemByHeightDto,
  ): Promise<BlockHeaderDto> {
    return this.appService.getHeaderByHeight(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.height,
    );
  }

  @ApiResponse({
    schema: { type: 'integer', format: 'int64' },
  })
  @Get('height')
  async getHeight(@Query() queryParams: GetHeightDto): Promise<number> {
    return this.appService.getHeightWithDepth(
      queryParams.host,
      queryParams.network,
      queryParams.depth,
    );
  }

  @ApiResponse({
    isArray: true,
    type: TransactionElementDto,
  })
  @Get('txs')
  async getTxs(
    @Query() queryParams: GetItemsDto,
  ): Promise<TransactionElementDto[]> {
    return this.appService.getTxs(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.from,
      queryParams.to,
    );
  }

  @ApiResponse({
    isArray: true,
    type: TransactionElementDto,
  })
  @Get('tx_by_hash')
  async getTxByHash(
    @Query() queryParams: GetItemByHashDto,
  ): Promise<TransactionElementDto[]> {
    return this.appService.getTxByHash(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.hash,
    );
  }

  @ApiResponse({
    isArray: true,
    type: TransactionElementDto,
  })
  @Get('tx_by_height')
  async getTxByHeight(
    @Query() queryParams: GetItemByHeightDto,
  ): Promise<TransactionElementDto[]> {
    return this.appService.getTxByHeight(
      queryParams.host,
      queryParams.network,
      queryParams.chain_id,
      queryParams.height,
    );
  }
}

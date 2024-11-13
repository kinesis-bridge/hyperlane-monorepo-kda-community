import { AppService } from './app.service';
import { BuildPactTxDto } from './dto/in/build-pact-tx.dto';
import { ContinueTransferRemoteDto } from './dto/in/continue-transfer-remote.dto';
import { GetHeightDto } from './dto/in/get-height.dto';
import { GetItemByHashDto } from './dto/in/get-item-by-hash.dto';
import { GetItemByHeightDto } from './dto/in/get-item-by-height.dto';
import { GetItemsDto } from './dto/in/get-items.dto';
import { LocalRequestBodyDto } from './dto/in/local-request-body.dto';
import { PollRequestBodyDto } from './dto/in/poll-request-body.dto';
import { SendRequestBodyDto } from './dto/in/send-request-body';
import { BlockHeaderDto } from './dto/out/block-header.dto';
import { BlockPayloadsDto } from './dto/out/block-payloads.dto';
import { CommandResultDto } from './dto/out/command-result.dto';
import { CommandDto } from './dto/out/command.dto';
import { EventDataDto } from './dto/out/event-data.dto';
import { PollResponseDto } from './dto/out/poll-response.dto';
import { RequestKeysDto } from './dto/out/request-keys.dto';
import { TransactionElementDto } from './dto/out/transaction-element.dto';
import { ChainId, ICommandResult, IUnsignedCommand } from '@kadena/client';
import { Body, Controller, Get, Post, Query } from '@nestjs/common';
import { ApiExtraModels, ApiResponse, getSchemaPath } from '@nestjs/swagger';

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
    type: CommandDto,
  })
  @Post('build_pact_tx')
  async buildTx(@Body() buildPactTxDto: BuildPactTxDto): Promise<CommandDto> {
    const unsignedCommand = await this.appService.buildPactTx(
      buildPactTxDto.host,
      buildPactTxDto.network,
      buildPactTxDto.chain_id,
      buildPactTxDto.pactCode,
      buildPactTxDto.signer,
      buildPactTxDto.senderAccount,
      buildPactTxDto.gasLimit,
      buildPactTxDto.verifiers || [],
    );

    return new CommandDto(unsignedCommand);
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

  @ApiExtraModels(CommandResultDto)
  @ApiResponse({
    schema: {
      type: 'object',
      additionalProperties: { $ref: getSchemaPath(CommandResultDto) },
    },
  })
  @Post('poll')
  async poll(@Body() body: PollRequestBodyDto): Promise<PollResponseDto> {
    return this.appService.poll(body, body.hostapi);
  }

  @ApiResponse({
    type: RequestKeysDto,
  })
  @Post('send')
  async send(@Body() body: SendRequestBodyDto): Promise<RequestKeysDto> {
    return this.appService.send(body, body.hostapi);
  }

  @ApiResponse({
    type: CommandResultDto,
  })
  @Post('continue_transfer_remote')
  async continueTransferRemote(
    @Body() body: ContinueTransferRemoteDto,
  ): Promise<ICommandResult> {
    return this.appService.continueTransferRemote(
      body.host,
      body.network,
      body.chain_id,
      body.pactId,
      body.destinationChainId.toString() as ChainId,
      body.step,
      body.rollback,
    );
  }

  @ApiResponse({
    type: CommandResultDto,
  })
  @Post('local')
  async local(@Body() body: LocalRequestBodyDto): Promise<CommandResultDto> {
    return this.appService.local(
      body.cmd,
      body.hostapi,
      body.preflight,
      body.signatureVerification,
    );
  }
}

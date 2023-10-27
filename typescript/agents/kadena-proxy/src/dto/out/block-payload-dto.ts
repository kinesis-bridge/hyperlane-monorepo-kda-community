import { ApiProperty } from '@nestjs/swagger';
import { CoinbaseDto } from './coinbase.dto';
import { MinerDataDto } from './miner-data.dto';
import { TransactionElementDto } from './transaction-element.dto';

export class BlockPayloadDto {
  @ApiProperty()
  minerData: MinerDataDto;
  @ApiProperty()
  coinbase: CoinbaseDto;
  @ApiProperty({ isArray: true, type: TransactionElementDto })
  transactions: TransactionElementDto[];
  @ApiProperty()
  payloadHash: string;
  @ApiProperty()
  transactionsHash: string;
  @ApiProperty()
  outputsHash: string;
}

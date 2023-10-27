import { ApiPropertyOptional, ApiProperty } from '@nestjs/swagger';
import { CoinbaseDto } from './coinbase.dto';
import { TransactionPayloadDto } from './transaction-payload.dto';

export class TransactionElementDto {
  @ApiPropertyOptional({ type: 'integer', format: 'int64', minimum: 0 })
  height?: number;
  @ApiProperty()
  transaction: TransactionPayloadDto;
  @ApiProperty()
  output: CoinbaseDto;
}

import { ApiProperty } from '@nestjs/swagger';
import { ResultDto } from './result.dto';
import { EventDataDto } from './event-data.dto';

export class CoinbaseDto {
  @ApiProperty({ minimum: 0 })
  gas: number;
  @ApiProperty()
  result: ResultDto;
  @ApiProperty()
  reqKey: string;
  @ApiProperty()
  logs: string;
  @ApiProperty({ isArray: true, type: EventDataDto })
  events: EventDataDto[];
  @ApiProperty({ nullable: true })
  metaData: any;
  @ApiProperty({ nullable: true })
  continuation: any;
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  txId: number;
}

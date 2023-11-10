import { ApiProperty, ApiPropertyOptional } from '@nestjs/swagger';
import { PactEventDto } from './pact-event.dto';
import { ChainwebResponseMetaDataDto } from './chainweb-response-meta-data.dto';
import { PactExecDto } from './pact-exec.dto';
import { PactResultErrorDto } from './pact-result-error.dto';
import { PactResultSuccessDto } from './pact-result-success.dto';

export class CommandResultDto {
  @ApiProperty()
  reqKey: string;
  @ApiProperty({ nullable: true })
  txId: number | null;
  @ApiProperty()
  result: PactResultSuccessDto | PactResultErrorDto;
  @ApiProperty()
  gas: number;
  @ApiProperty({ nullable: true })
  logs: string | null;
  @ApiProperty({ nullable: true })
  continuation: PactExecDto | null;
  @ApiProperty({ nullable: true })
  metaData: ChainwebResponseMetaDataDto | null;
  @ApiPropertyOptional({ isArray: true, type: PactEventDto })
  events?: Array<PactEventDto>;
}

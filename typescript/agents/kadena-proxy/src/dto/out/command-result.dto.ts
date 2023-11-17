import {
  ApiExtraModels,
  ApiProperty,
  ApiPropertyOptional,
  getSchemaPath,
} from '@nestjs/swagger';
import { PactEventDto } from './pact-event.dto';
import { ChainwebResponseMetaDataDto } from './chainweb-response-meta-data.dto';
import { PactExecDto } from './pact-exec.dto';
import { PactResultErrorDto } from './pact-result-error.dto';
import { PactResultSuccessDto } from './pact-result-success.dto';

@ApiExtraModels(PactResultSuccessDto, PactResultErrorDto)
export class CommandResultDto {
  @ApiProperty()
  reqKey: string;
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0, nullable: true })
  txId: number | null;
  @ApiProperty({
    oneOf: [
      { $ref: getSchemaPath(PactResultSuccessDto) },
      { $ref: getSchemaPath(PactResultErrorDto) },
    ],
  })
  result: PactResultSuccessDto | PactResultErrorDto;
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
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

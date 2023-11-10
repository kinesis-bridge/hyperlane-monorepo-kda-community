import { ApiProperty } from '@nestjs/swagger';
import { PactValue } from './poll-response.dto';

export class PactExecDto {
  @ApiProperty()
  pactId: string;
  @ApiProperty()
  step: number;
  @ApiProperty()
  stepCount: number;
  @ApiProperty({ nullable: true })
  executed: boolean | null;
  @ApiProperty()
  stepHasRollback: boolean;
  @ApiProperty()
  continuation: {
    def: string;
    args: PactValue;
  };
  @ApiProperty({ nullable: true })
  yield: {
    data: Array<[string, PactValue]>;
    provenance: {
      targetChainId: string | number;
      moduleHash: string;
    } | null;
  } | null;
}

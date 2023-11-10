import { ApiProperty } from '@nestjs/swagger';
import { PactValue } from './poll-response.dto';

export class PactEventDto {
  @ApiProperty()
  name: string;
  @ApiProperty()
  module: {
    name: string;
    namespace: string | null;
  };
  @ApiProperty({ isArray: true })
  params: Array<PactValue>;
  @ApiProperty()
  moduleHash: string;
}

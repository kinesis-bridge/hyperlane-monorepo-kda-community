import { ApiProperty } from '@nestjs/swagger';

export class MinerDataDto {
  @ApiProperty()
  account: string;
  @ApiProperty()
  predicate: string;
  @ApiProperty()
  'public-keys': string[];
}

import { ApiProperty } from '@nestjs/swagger';

export class CutHashDto {
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  height: number;
  @ApiProperty()
  hash: string;
}

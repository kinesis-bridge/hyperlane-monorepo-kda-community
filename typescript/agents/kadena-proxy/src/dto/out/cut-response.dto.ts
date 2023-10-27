import { ApiProperty } from '@nestjs/swagger';
import { CutHashDto } from './cut-hash.dto';

export class CutResponseDto {
  @ApiProperty({ nullable: true })
  origin: any;
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  height: number;
  @ApiProperty()
  weight: string;
  @ApiProperty()
  hashes: {
    [key: string]: CutHashDto;
  };
  @ApiProperty()
  id: string;
  @ApiProperty()
  instance: string;
}

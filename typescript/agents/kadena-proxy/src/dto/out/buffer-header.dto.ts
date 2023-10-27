import { ApiProperty } from '@nestjs/swagger';
import { BlockHeaderDto } from './block-header.dto';

export class BufferHeaderDto {
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  txCount: number;
  @ApiProperty()
  powHash: string;
  @ApiProperty()
  header: BlockHeaderDto;
  @ApiProperty()
  target: string;
}

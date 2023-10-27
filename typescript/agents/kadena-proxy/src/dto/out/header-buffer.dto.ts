import { ApiProperty } from '@nestjs/swagger';
import { BufferHeaderDto } from './buffer-header.dto';

export class HeaderBufferDto {
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  depth: number;
  @ApiProperty()
  callback: (header: BufferHeaderDto) => void;
  @ApiProperty({ nullable: true, type: 'integer', format: 'int64', minimum: 0 })
  curHeight: number;
  @ApiProperty({ isArray: true, type: BufferHeaderDto })
  buffer: BufferHeaderDto[];
  @ApiProperty()
  add: (u: BufferHeaderDto) => void;
}

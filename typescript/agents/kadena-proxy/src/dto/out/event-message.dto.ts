import { ApiProperty } from '@nestjs/swagger';
import { BufferHeaderDto } from './buffer-header.dto';

export class EventMessageDto {
  @ApiProperty()
  type: string;
  @ApiProperty()
  data: BufferHeaderDto;
  @ApiProperty()
  lastEventId: string;
  @ApiProperty()
  origin: string;
}

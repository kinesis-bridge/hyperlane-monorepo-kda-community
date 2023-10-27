import { ApiPropertyOptional, ApiProperty } from '@nestjs/swagger';
import { ModuleDto } from './module.dto';

export class EventDataDto {
  @ApiPropertyOptional({ type: 'integer', format: 'int64', minimum: 0 })
  height?: number;
  @ApiProperty({
    isArray: true,
  })
  params: Array<number | string>;
  @ApiProperty()
  name: string;
  @ApiProperty()
  module: ModuleDto;
  @ApiProperty()
  moduleHash: string;
}

import { ApiPropertyOptional, ApiProperty } from '@nestjs/swagger';

export class PagedResponseDto<T> {
  @ApiPropertyOptional()
  next?: string;
  @ApiProperty()
  items: T[];
  @ApiPropertyOptional({ type: 'integer', format: 'int32', minimum: 0 })
  limit?: number;
}

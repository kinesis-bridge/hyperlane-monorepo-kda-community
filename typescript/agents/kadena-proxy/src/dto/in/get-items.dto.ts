import { ApiProperty } from '@nestjs/swagger';
import { Type } from 'class-transformer';
import { Min, IsNotEmpty, IsInt } from 'class-validator';
import { ProviderDto } from './provider.dto';

export class GetItemsDto extends ProviderDto {
  @IsInt()
  @Min(0)
  @IsNotEmpty()
  @Type(() => Number)
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  from: number;
  @IsInt()
  @Min(0)
  @IsNotEmpty()
  @Type(() => Number)
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  to: number;
}

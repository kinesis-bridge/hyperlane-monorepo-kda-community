import { ApiProperty } from '@nestjs/swagger';
import { Type } from 'class-transformer';
import { Min, IsNotEmpty, IsInt } from 'class-validator';
import { ProviderDto } from './provider.dto';

export class GetItemByHeightDto extends ProviderDto {
  @IsInt()
  @Min(0)
  @IsNotEmpty()
  @Type(() => Number)
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  height: number;
}

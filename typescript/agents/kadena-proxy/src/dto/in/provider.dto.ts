import { ApiProperty } from '@nestjs/swagger';
import { Type } from 'class-transformer';
import { Min, IsUrl, IsNotEmpty, IsInt, IsString } from 'class-validator';

export class ProviderDto {
  @IsUrl({ require_tld: false })
  @ApiProperty()
  host: string;
  @IsString()
  @IsNotEmpty()
  @ApiProperty()
  network: string;
  @IsInt()
  @Min(0)
  @IsNotEmpty()
  @Type(() => Number)
  @ApiProperty({ type: 'integer', format: 'int32', minimum: 0 })
  chain_id: number;
}

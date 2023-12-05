import { ApiProperty, ApiPropertyOptional } from '@nestjs/swagger';
import { Type } from 'class-transformer';
import {
  Min,
  IsUrl,
  IsNotEmpty,
  IsInt,
  IsString,
  IsOptional,
} from 'class-validator';

export class GetHeightDto {
  @IsUrl({ require_tld: false })
  @ApiProperty()
  host: string;
  @IsString()
  @IsNotEmpty()
  @ApiProperty()
  network: string;
  @IsInt()
  @Min(0)
  @IsOptional()
  @IsNotEmpty()
  @Type(() => Number)
  @ApiPropertyOptional({
    type: 'integer',
    format: 'int64',
    minimum: 0,
    default: 0,
  })
  depth: number = 0;
}

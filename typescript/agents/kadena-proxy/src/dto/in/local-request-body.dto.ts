import { CommandDto } from '../out/command.dto';
import { ApiProperty } from '@nestjs/swagger';
import { IsNotEmpty, IsNumber, IsOptional, IsUrl, Min } from 'class-validator';

export class LocalRequestBodyDto {
  @ApiProperty()
  @IsNotEmpty()
  cmd: CommandDto;

  @ApiProperty()
  @IsNotEmpty()
  @IsUrl({ require_tld: false })
  hostapi: string;

  @ApiProperty()
  @IsNotEmpty()
  preflight: boolean;

  @ApiProperty()
  @IsNotEmpty()
  signatureVerification: boolean;

  @ApiProperty({ required: false, default: 0 })
  @IsOptional()
  @IsNumber()
  @Min(0)
  rewindDepth: number = 0;
}

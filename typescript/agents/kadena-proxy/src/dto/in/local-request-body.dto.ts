import { ApiProperty } from '@nestjs/swagger';
import { CommandDto } from '../out/command.dto';
import { IsNotEmpty, IsUrl } from 'class-validator';

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
}

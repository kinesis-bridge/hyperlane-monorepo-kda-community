import { ApiProperty } from '@nestjs/swagger';
import { CommandDto } from '../out/command.dto';
import { IsNotEmpty } from 'class-validator';

export class LocalRequestBodyDto {
  @ApiProperty()
  @IsNotEmpty()
  cmd: CommandDto;
  @ApiProperty()
  @IsNotEmpty()
  hostapi: string;
  @ApiProperty()
  @IsNotEmpty()
  preflight: boolean;
  @ApiProperty()
  @IsNotEmpty()
  signatureVerification: boolean;
}

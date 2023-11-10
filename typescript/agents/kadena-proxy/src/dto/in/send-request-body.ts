import { ApiProperty } from '@nestjs/swagger';
import { UnsignedCommandDto } from '../out/unsigned-command.dto';
import { IsNotEmpty } from 'class-validator';

export class SendRequestBodyDto {
  @ApiProperty({ isArray: true, type: UnsignedCommandDto })
  @IsNotEmpty()
  cmds: Array<UnsignedCommandDto>;
  @ApiProperty()
  @IsNotEmpty()
  hostapi: string;
}

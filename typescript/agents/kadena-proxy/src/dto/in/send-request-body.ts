import { ApiProperty } from '@nestjs/swagger';
import { CommandDto } from '../out/command.dto';
import { IsNotEmpty, IsUrl } from 'class-validator';

export class SendRequestBodyDto {
  @ApiProperty({ isArray: true, type: CommandDto })
  @IsNotEmpty()
  cmds: Array<CommandDto>;
  @ApiProperty()
  @IsNotEmpty()
  @IsUrl({ require_tld: false })
  hostapi: string;
}

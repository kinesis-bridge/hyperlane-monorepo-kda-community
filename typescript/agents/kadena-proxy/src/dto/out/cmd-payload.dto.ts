import { ApiProperty } from '@nestjs/swagger';
import { ExecDto } from './exec.dto';

export class CmdPayloadDto {
  @ApiProperty()
  exec: ExecDto;
}

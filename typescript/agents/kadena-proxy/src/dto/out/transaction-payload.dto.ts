import { ApiProperty } from '@nestjs/swagger';
import { CmdDto } from './cmd.dto';
import { SigDto } from './sig.dto';

export class TransactionPayloadDto {
  @ApiProperty()
  hash: string;
  @ApiProperty({ isArray: true, type: SigDto })
  sigs: SigDto[];
  @ApiProperty()
  cmd: CmdDto;
}

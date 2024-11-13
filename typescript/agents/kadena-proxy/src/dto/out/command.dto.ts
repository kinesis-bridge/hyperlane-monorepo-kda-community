import { SignatureJsonDto } from './signature-json.dto';
import { IUnsignedCommand } from '@kadena/types';
import { ApiProperty } from '@nestjs/swagger';

export class CommandDto {
  @ApiProperty()
  cmd: string;
  @ApiProperty()
  hash: string;
  @ApiProperty({ isArray: true, type: SignatureJsonDto })
  sigs: Array<SignatureJsonDto>;

  constructor(unsignedCommand: IUnsignedCommand) {
    this.cmd = unsignedCommand.cmd;
    this.hash = unsignedCommand.hash;
    this.sigs = unsignedCommand.sigs.map((sig) => {
      if (sig && 'sig' in sig && sig.sig) {
        return { sig: sig.sig };
      } else {
        return { sig: '' };
      }
    });
  }
}

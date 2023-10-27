import { ApiProperty } from '@nestjs/swagger';
import { CmdPayloadDto } from './cmd-payload.dto';
import { SignerDto } from './signer.dto';
import { MetaDto } from './meta.dto';

export class CmdDto {
  @ApiProperty()
  networkId: string;
  @ApiProperty()
  payload: CmdPayloadDto;
  @ApiProperty({ isArray: true, type: SignerDto })
  signers: SignerDto[];
  @ApiProperty()
  meta: MetaDto;
  @ApiProperty()
  nonce: string;
}

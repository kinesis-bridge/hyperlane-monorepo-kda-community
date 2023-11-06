import { ApiProperty } from '@nestjs/swagger';
import { SignatureJsonDto } from './signature-json.dto';

export class UnsignedCommandDto {
  @ApiProperty()
  cmd: string;
  @ApiProperty()
  hash: string;
  @ApiProperty({ isArray: true, type: SignatureJsonDto })
  sigs: Array<SignatureJsonDto>;
}

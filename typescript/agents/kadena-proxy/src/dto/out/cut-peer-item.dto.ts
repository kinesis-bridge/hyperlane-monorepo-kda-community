import { ApiProperty } from '@nestjs/swagger';
import { CutPeerAddressDto } from './cut-peer-address.dto';

export class CutPeerItemDto {
  @ApiProperty()
  address: CutPeerAddressDto;
  @ApiProperty({ nullable: true })
  id: string;
}

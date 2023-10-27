import { ApiProperty } from '@nestjs/swagger';

export class CutPeerAddressDto {
  @ApiProperty()
  hostname: string;
  @ApiProperty({ type: 'integer', format: 'int32', minimum: 0 })
  port: number;
}

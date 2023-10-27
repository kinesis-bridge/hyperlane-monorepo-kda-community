import { ApiProperty } from '@nestjs/swagger';

export class SignerDto {
  @ApiProperty()
  pubKey: string;
}

import { ApiProperty } from '@nestjs/swagger';

export class SignatureJsonDto {
  @ApiProperty({ nullable: true })
  sig: string;
}

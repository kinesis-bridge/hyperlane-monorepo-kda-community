import { ApiProperty } from '@nestjs/swagger';

export class SigDto {
  @ApiProperty()
  sig: string;
}

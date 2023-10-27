import { ApiProperty } from '@nestjs/swagger';

export class KeysetDto {
  @ApiProperty()
  pred: string;
  @ApiProperty()
  keys: string[];
}

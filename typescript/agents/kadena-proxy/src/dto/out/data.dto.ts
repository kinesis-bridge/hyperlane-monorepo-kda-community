import { ApiProperty } from '@nestjs/swagger';
import { KeysetDto } from './keyset.dto';

export class DataDto {
  @ApiProperty()
  keyset: KeysetDto;
}

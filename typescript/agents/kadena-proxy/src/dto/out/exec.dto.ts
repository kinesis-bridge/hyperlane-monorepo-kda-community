import { ApiProperty } from '@nestjs/swagger';
import { DataDto } from './data.dto';

export class ExecDto {
  @ApiProperty()
  data: DataDto;
  @ApiProperty()
  code: string;
}

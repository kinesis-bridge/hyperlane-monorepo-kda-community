import { ApiProperty } from '@nestjs/swagger';

export class ResultDto {
  @ApiProperty()
  status: string;
  @ApiProperty()
  data: string;
}

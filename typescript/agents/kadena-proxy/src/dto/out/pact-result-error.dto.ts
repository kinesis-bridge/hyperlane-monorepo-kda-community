import { ApiProperty } from '@nestjs/swagger';

export class PactResultErrorDto {
  @ApiProperty()
  status: string;
  @ApiProperty()
  error: any;
}

import { ApiProperty } from '@nestjs/swagger';

export class PactResultSuccessDto {
  @ApiProperty()
  status: string;
  @ApiProperty()
  data: any;
}

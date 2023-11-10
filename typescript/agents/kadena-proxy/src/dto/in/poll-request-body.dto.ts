import { ApiProperty } from '@nestjs/swagger';
import { IsNotEmpty } from 'class-validator';

export class PollRequestBodyDto {
  @ApiProperty({ isArray: true, type: String })
  @IsNotEmpty()
  requestKeys: Array<string>;
  @ApiProperty()
  @IsNotEmpty()
  hostapi: string;
}

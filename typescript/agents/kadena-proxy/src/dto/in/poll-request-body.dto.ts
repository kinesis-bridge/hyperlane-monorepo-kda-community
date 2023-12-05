import { ApiProperty } from '@nestjs/swagger';
import { IsNotEmpty, IsUrl } from 'class-validator';

export class PollRequestBodyDto {
  @ApiProperty({ isArray: true, type: String })
  @IsNotEmpty()
  requestKeys: Array<string>;
  @ApiProperty()
  @IsNotEmpty()
  @IsUrl({ require_tld: false })
  hostapi: string;
}

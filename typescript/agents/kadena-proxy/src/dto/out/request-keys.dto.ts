import { ApiProperty } from '@nestjs/swagger';
import { IsNotEmpty } from 'class-validator';

export class RequestKeysDto {
  @ApiProperty({ isArray: true, type: String })
  @IsNotEmpty()
  requestKeys: Array<string>;
}

import { ApiProperty } from '@nestjs/swagger';
import { IsNotEmpty, IsString } from 'class-validator';
import { ProviderDto } from './provider.dto';

export class GetItemByHashDto extends ProviderDto {
  @IsString()
  @IsNotEmpty()
  @ApiProperty()
  hash: string;
}

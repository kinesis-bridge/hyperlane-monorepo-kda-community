import { ApiProperty } from '@nestjs/swagger';
import { IsNotEmpty, IsString } from 'class-validator';
import { ProviderDto } from './provider.dto';

export class BuildPactTxDto extends ProviderDto {
  @IsNotEmpty()
  @IsString()
  @ApiProperty()
  pactCode: string;
  @IsString()
  @ApiProperty()
  signer: string;
  @IsString()
  @ApiProperty()
  senderAccount: string;
}

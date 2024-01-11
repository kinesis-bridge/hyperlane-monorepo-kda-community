import { ApiProperty } from '@nestjs/swagger';
import { IsInt, IsNotEmpty, IsString, Min } from 'class-validator';
import { ProviderDto } from './provider.dto';
import { Type } from 'class-transformer';

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
  @IsInt()
  @Min(0)
  @IsNotEmpty()
  @Type(() => Number)
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  gasLimit: number;
}

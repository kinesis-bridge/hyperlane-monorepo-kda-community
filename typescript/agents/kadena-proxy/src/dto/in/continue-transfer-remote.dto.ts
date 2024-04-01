import { ApiProperty } from '@nestjs/swagger';
import { IsBoolean, IsInt, IsNotEmpty, IsString, Min } from 'class-validator';
import { ProviderDto } from './provider.dto';

export class ContinueTransferRemoteDto extends ProviderDto {
  @IsNotEmpty()
  @IsString()
  @ApiProperty({ description: 'The pact ID' })
  pactId: string;

  @IsNotEmpty()
  @IsInt()
  @Min(0)
  @ApiProperty({ description: 'The destination chain ID' })
  destinationChainId: number;

  @IsNotEmpty()
  @IsInt()
  @Min(0)
  @ApiProperty({ description: 'The step number', minimum: 0 })
  step: number;

  @IsNotEmpty()
  @IsBoolean()
  @ApiProperty({
    description: 'Indicates if the operation should be rolled back',
  })
  rollback: boolean;
}

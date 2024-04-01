import { ApiProperty } from '@nestjs/swagger';
import {
  IsArray,
  IsInt,
  IsNotEmpty,
  IsOptional,
  IsString,
  Min,
} from 'class-validator';
import { ProviderDto } from './provider.dto';
import { Type } from 'class-transformer';

export class Verifier {
  @IsNotEmpty()
  @IsString()
  @ApiProperty({ description: 'The verifier name' })
  name: string;

  @IsNotEmpty()
  @ApiProperty({ description: 'The verifier proof' })
  proof: any; // replace with the actual type and add appropriate validation

  @IsArray()
  @ApiProperty({ type: [Array], description: 'The verifier capabilities' })
  capabilities: any[][]; // replace with the actual type and add appropriate validation
}

export class BuildPactTxDto extends ProviderDto {
  @IsNotEmpty()
  @IsString()
  @ApiProperty({ description: 'The pact code' })
  pactCode: string;

  @IsNotEmpty()
  @IsString()
  @ApiProperty({ description: 'The signer' })
  signer: string;

  @IsNotEmpty()
  @IsString()
  @ApiProperty({ description: 'The sender account' })
  senderAccount: string;

  @IsNotEmpty()
  @IsInt()
  @Min(0)
  @Type(() => Number)
  @ApiProperty({
    type: 'integer',
    format: 'int64',
    minimum: 0,
    description: 'The gas limit',
  })
  gasLimit: number;

  @IsOptional()
  @IsArray()
  @Type(() => Verifier)
  @ApiProperty({
    required: false,
    type: [Verifier],
    description: 'The verifiers',
  })
  verifiers: Verifier[];
}

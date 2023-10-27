import { ApiProperty } from '@nestjs/swagger';

export class MetaDto {
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  creationTime: number;
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  ttl: number;
  @ApiProperty({ minimum: 0 })
  gasLimit: number;
  @ApiProperty()
  chainId: string;
  @ApiProperty({ minimum: 0 })
  gasPrice: number;
  @ApiProperty()
  sender: string;
}

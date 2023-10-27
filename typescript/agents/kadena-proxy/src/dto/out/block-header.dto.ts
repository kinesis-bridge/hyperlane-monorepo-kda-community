import { ApiProperty } from '@nestjs/swagger';

export class BlockHeaderDto {
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  creationTime: number;
  @ApiProperty()
  parent: string;
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  height: number;
  @ApiProperty()
  hash: string;
  @ApiProperty({ type: 'integer', format: 'int32', minimum: 0 })
  chainId: number;
  @ApiProperty()
  weight: string;
  @ApiProperty({ type: 'integer', format: 'int32', minimum: 0 })
  featureFlags: number;
  @ApiProperty({ type: 'integer', format: 'int64', minimum: 0 })
  epochStart: number;
  @ApiProperty({
    type: Object,
    additionalProperties: { type: 'string' },
  })
  adjacents: {
    [key: string]: string;
  };
  @ApiProperty()
  payloadHash: string;
  @ApiProperty()
  chainwebVersion: string;
  @ApiProperty()
  target: string;
  @ApiProperty()
  nonce: string;
}

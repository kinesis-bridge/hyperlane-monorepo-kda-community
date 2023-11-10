import { ApiProperty } from '@nestjs/swagger';

export class MetaDataDto {
  @ApiProperty()
  creationTime: number;
  @ApiProperty()
  ttl: number;
  @ApiProperty()
  gasLimit: number;
  @ApiProperty()
  gasPrice: number;
  @ApiProperty()
  sender: string;
  @ApiProperty()
  chainId: string | number;
}

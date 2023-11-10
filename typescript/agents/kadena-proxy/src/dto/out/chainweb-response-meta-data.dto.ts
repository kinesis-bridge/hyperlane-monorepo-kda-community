import { ApiProperty, ApiPropertyOptional } from '@nestjs/swagger';
import { MetaDataDto } from './meta-data.dto';

export class ChainwebResponseMetaDataDto {
  @ApiProperty()
  blockHash: string;
  @ApiProperty()
  blockTime: number;
  @ApiProperty()
  blockHeight: number;
  @ApiProperty()
  prevBlockHash: string;
  @ApiPropertyOptional()
  publicMeta?: MetaDataDto;
}

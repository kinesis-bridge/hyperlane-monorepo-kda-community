import { ApiProperty } from '@nestjs/swagger';
import { BlockHeaderDto } from './block-header.dto';
import { BlockPayloadDto } from './block-payload-dto';

// This DTO corresponds IBlockPayloads<IBlockPayload<ITransactionPayload>>.
export class BlockPayloadsDto {
  @ApiProperty()
  header: BlockHeaderDto;
  @ApiProperty()
  payload: BlockPayloadDto;
}

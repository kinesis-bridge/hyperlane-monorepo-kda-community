import { ApiProperty } from '@nestjs/swagger';

export class ModuleDto {
  @ApiProperty({ nullable: true })
  namespace: any;
  @ApiProperty()
  name: string;
}

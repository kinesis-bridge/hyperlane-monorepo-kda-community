import { ApiProperty } from '@nestjs/swagger';

export class OnFailedAttemptDto extends Error {
  @ApiProperty()
  message: string;
}

import { ApiPropertyOptional } from '@nestjs/swagger';
import { OnFailedAttemptDto } from './on-failed-attempt.dto';

export class RetryOptionsDto {
  @ApiPropertyOptional()
  onFailedAttempt?: (x: OnFailedAttemptDto) => void;
  @ApiPropertyOptional({ type: 'integer', format: 'int32', minimum: 0 })
  retries?: number;
  @ApiPropertyOptional({ type: 'integer', format: 'int32', minimum: 0 })
  minTimeout?: number;
  @ApiPropertyOptional()
  randomize?: boolean;
  @ApiPropertyOptional()
  retry404?: boolean;
}

import { CommandResultDto } from './command-result.dto';

export class PollResponseDto {
  [key: string]: CommandResultDto;
}

export type PactValue = any;

import { CommandResultDto } from './command-result.dto';
import type { IBase64Url } from '@kadena/types';

export class PollResponseDto {
  [key: IBase64Url]: CommandResultDto;
}

export type PactValue = any;

import { IsNotEmpty, IsNumber, IsString, Min } from 'class-validator';

export class AppConfig {
  @IsNumber()
  @Min(0)
  CONT_CONFIRMATION_DEPTH: number;

  @IsNumber()
  @Min(0)
  POLL_TIMEOUT: number;

  @IsNumber()
  @Min(0)
  POLL_INTERVAL: number;

  @IsString()
  @IsNotEmpty()
  KADENA_GAS_STATION_MODULE_NAME: string;

  @IsNumber()
  @Min(0)
  GAS_STATION_GAS_LIMIT: number;

  @IsNumber()
  @Min(0)
  GAS_STATION_GAS_PRICE: number;

  @IsString()
  @IsNotEmpty()
  GAS_STATION_PAYER: string;
}

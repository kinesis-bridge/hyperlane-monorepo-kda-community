import { Controller, Get, Query } from '@nestjs/common';
import { AppService } from './app.service';
import { IBlockHeader, ICutResponse } from '@kadena/chainwebjs/lib/types';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Get('cuts')
  async getCurrentCuts(): Promise<ICutResponse> {
    return this.appService.getCurrentCut();
  }

  @Get('recent-headers')
  async getCurrentHeader(): Promise<IBlockHeader[]> {
    return this.appService.getBlockHeader();
  }

  @Get('tx')
  async buildTx() {
    return this.appService.buildTx();
  }
}

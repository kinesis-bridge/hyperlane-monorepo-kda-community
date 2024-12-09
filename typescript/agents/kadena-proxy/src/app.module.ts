import { AppController } from './app.controller';
import { AppService } from './app.service';
import { TrimHostMiddleware } from './trim-host.middleware';
import { Module, NestModule, MiddlewareConsumer } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
    }),
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule implements NestModule {
  configure(consumer: MiddlewareConsumer) {
    consumer.apply(TrimHostMiddleware).forRoutes(AppController);
  }
}
